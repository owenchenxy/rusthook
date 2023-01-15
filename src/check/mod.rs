use core::panic;
use std::{collections::HashMap, path::Path, fs::{self, File}, io::{Error, ErrorKind, self}, fmt::format };

use log::{LevelFilter, Log, Level, MetadataBuilder, Metadata, logger};
use simplelog::SharedLogger;

use crate::{Configs, config::Config, command::is_valid_command};

pub fn is_webhook_id_in_configs(configs: &Configs, http_request: &HashMap<String, String>) -> Result<(), io::Error>{
    let requested_id = http_request.get("Id").unwrap().trim_start_matches("/").to_string();
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
    if !is_valid_command(execute_command, command_working_directory).unwrap(){
        let err_msg = format!("invalid command: {} (working directory: {})", &config.execute_command, &config.command_working_directory);
        log::error!("{}", err_msg);

        let error = io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid Command",
        );
        return Err(error);
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

pub fn preflight_check(config: &Config) -> Result<(), io::Error>{
    check_execute_command(&config)?;
    check_log_config(&config)?;
    Ok(())
}


#[test]
#[should_panic]
fn test_isnot_webhook_id_in_configs(){
    let configs = Configs::new("src/config/hooks.test.yaml");
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Id".to_string(), "webhook-test-3".to_string());
    let _ = is_webhook_id_in_configs(&configs, &http_request);
}

#[test]
fn test_is_webhook_id_in_configs(){
    let configs = Configs::new("src/config/hooks.test.yaml");
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Id".to_string(), "webhook-test-1".to_string());
    let _ = is_webhook_id_in_configs(&configs, &http_request);
}

#[test]
fn test_check_log_config(){
    let mut config = Config::new();
    config.log_dir = String::from("./log_dir/testhook/");
    let _ = check_log_config(&config);
}

