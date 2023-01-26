use std::{collections::HashMap, fs, io};
use crate::{Configs, config::Config, command::is_valid_command, parser::parse_hook_id_from_url};

pub fn is_webhook_id_in_configs(configs: &Configs, http_request: &HashMap<String, String>) -> Result<(), io::Error>{
    let url = http_request.get("Url").unwrap();
    let requested_id = parse_hook_id_from_url(url);
    let required_ids:Vec<String> = configs.get_webhook_ids()
    .iter()
    .map(|id|id.trim_start_matches("/").to_string())
    .collect();


    if !required_ids.contains(&requested_id) {
        let err_msg = format!("request ignored with undefined hook id: {}", requested_id);
        log::warn!("{}", err_msg);

        let error = io::Error::new(
            io::ErrorKind::InvalidInput,
            "Undefined Hook Id",
        );
        return Err(error);
    }
    Ok(())
}

pub fn check_execute_command(config: &Config) -> io::Result<()>{
    let execute_command = &config.execute_command;
    let command_working_directory = &config.command_working_directory;
    match is_valid_command(execute_command, command_working_directory){
        Ok(b) => if !b{
            let err_msg = format!("invalid command: {} (working directory: {})", &config.execute_command, &config.command_working_directory);
            log::error!("{}", err_msg);
    
            let error = io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid Command",
            );
            return Err(error);
        },
        Err(e) => return Err(e)
    }
    Ok(())
}

pub fn check_log_config(config: &Config) -> io::Result<()>{
    match fs::create_dir_all(&config.log_dir) {
        Ok(_) => {
            log::info!("log directory is ready: {}", config.log_dir);
            Ok(())
        }
        Err(err) => {
            let err_msg = format!("accessing log directory failed, reason: {}", err);
            log::error!("{}", err_msg);

            let error = io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid Log Config",
            );
            Err(error)
        }
    }
}

pub fn check_trigger_rules(config: &Config, http_request: &HashMap<String, String>) -> io::Result<()>{
    // check trigger rules
    if let Some(r) = config.get_trigger_rule(){
        if !r.is_matched(&http_request){
            let err_msg = format!("Failed to Trigger Hook [{}]: Rule Mismatch!", config.id);
            log::error!("{}", err_msg);

            let error = io::Error::new(
                io::ErrorKind::InvalidInput,
                "Trigger Rules Mismatch",
            );
            return Err(error)
        }
    }
    log::info!("Trigger rules matched, hook [{}] triggered", config.id);
    Ok(())
}

pub fn preflight_check(config: &Config, http_request: &HashMap<String, String>) -> Result<(), io::Error>{
    check_execute_command(config)?;
    check_log_config(config)?;
    check_trigger_rules(config, http_request)?;
    Ok(())
}


#[test]
fn test_isnot_webhook_id_in_configs(){
    let config_file = format!("{}/src/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = Configs::new(&config_file);
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Url".to_string(), "/webhook-test-3/".to_string());
    let r = is_webhook_id_in_configs(&configs, &http_request);
    assert!(r.is_err());
}

#[test]
fn test_is_webhook_id_in_configs(){
    let config_file = format!("{}/src/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = Configs::new(&config_file);
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Url".to_string(), "/webhook-test-1/?a=1&b=2".to_string());
    let r = is_webhook_id_in_configs(&configs, &http_request);
    assert!(r.is_ok())
}

#[test]
fn test_check_log_config(){
    let mut config = Config::new();
    let log_dir = format!("{}/log_dir/testhook/", env!("CARGO_MANIFEST_DIR"));
    config.log_dir = String::from(&log_dir);
    let r = check_log_config(&config);
    if r.is_ok(){
        let log_dir = format!("{}/log_dir", env!("CARGO_MANIFEST_DIR"));
        let _ = fs::remove_dir_all(String::from(&log_dir));
    }
    assert!(r.is_ok());
}

