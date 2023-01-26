use std::{process::{Command, Child, Stdio}, str, collections::HashMap, io::{self, Write}};
use log::{Record, Log};

use crate::{config::{Config, configs::Configs}, response::{http_response_with_child, http_response_with_err}, arguments::Argument, mylog::{create_log_file, set_hook_logger}};

pub fn execute_script(cmd: &str, cwd: &str, stdout_log: &str, arguments: &Vec<String>) -> io::Result<Child>{
    let stdout_file = create_log_file(stdout_log).unwrap();
    let stderr_file = create_log_file(format!("{}.wf", stdout_log).as_str()).unwrap();
    let stdout = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);
    
    let stderr_log = format!("{}.wf", stdout_log);
    let hook_stdout_logger = set_hook_logger(stdout_log, &log::LevelFilter::Info);
    let hook_stderr_logger = set_hook_logger(stderr_log.as_str(), &log::LevelFilter::Info);

    let meta_args = format_args!("");
    let meta_record = Record::builder().args(meta_args).build();
    hook_stdout_logger.log(&meta_record);
    hook_stderr_logger.log(&meta_record);

    Command::new(cmd)
        .args(arguments)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
}

pub fn is_valid_command(command: &str, work_dir: &str) -> std::io::Result<bool>{
    match Command::new("sh")
    .arg("-c")
    .arg(format!("command -v {}", command))
    .current_dir(work_dir)//.arg(command_full_path.as_str())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status(){
        Ok(r) => Ok(r.success()),
        Err(e) => {
            let msg = format!("Invalid command [{}](dir: {}): {}", command, work_dir, e.to_string());
            log::error!("{}", msg);
            let error = io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid command",
            );
            Err(error)
        }
    } 
}

pub fn trigger_hook(config: &Config, http_request: &HashMap<String, String>) -> String{
    // find the right config from config file for the incoming request
    let arguments: Vec<String> = config.pass_arguments_to_command
    .iter()
    .map(|arg| match Argument::new(arg)
                                        .unwrap()
                                        .parse_from_request(http_request){
                                            Ok(r) => r,
                                            Err(_) => String::new()   
                                        }
    )
    .filter(| arg | !arg.is_empty())
    .collect();

    let stdout_log = config.get_log_path();
    let response = match execute_script(&config.execute_command, &config.command_working_directory, &stdout_log, &arguments){
        Ok(c) => {
            let msg = format!("Command [{}] issued under dir {} in process id: {}", &config.execute_command, &config.command_working_directory, c.id());
            log::info!("{}", msg);
            http_response_with_child(&c, http_request, config)
        },
        Err(e) => {
            let msg = format!("Failed to execute command: {}", e.to_string());
            log::error!("{}", msg);
            http_response_with_err(&e, http_request, Some(config))
        },
    };
    response
}

#[test]
fn test_execute_script(){
    let args = vec!["-a".to_string(), "-l".to_string()];
    let log = format!("{}/logs/test.log", env!("CARGO_MANIFEST_DIR"));
    let _ = execute_script("ls", "/", &log, &args);
}

#[test]
fn test_trigger_hook(){
    let config_file = format!("{}/src/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs: Configs = Configs::new(&config_file);
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Method".to_string(), "GET".to_string());
    http_request.insert("Url".to_string(), "/webhook-test-1/".to_string());
    http_request.insert("Host".to_string(), "127.0.0.1:7878".to_string());
    http_request.insert("Version".to_string(), "HTTP/1.1".to_string());
    let mut config = configs.get_config_by_http_request(&http_request);
    config.log_dir = format!("{}/logs", env!("CARGO_MANIFEST_DIR"));
    trigger_hook(&config, &http_request);
    ()
}

#[test]
#[should_panic]
fn test_isnot_valid_command(){
    assert!(is_valid_command("ks", "/").unwrap())
}

#[test]
fn test_is_valid_command(){
    assert!(is_valid_command("ls", "/").unwrap())
}

#[test]
fn test_is_valid_command_test_sh(){
    let work_dir = format!("{}/src/command/", env!("CARGO_MANIFEST_DIR"));
    let res = is_valid_command("./test.sh", &work_dir);
    assert!(res.unwrap())
}
