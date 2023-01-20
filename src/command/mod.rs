use std::{process::{Command, Child, Stdio, ExitStatus}, str, fs::File, collections::HashMap, env, io, path::{Path, PathBuf}};
use crate::{config::{configs::Configs, Config}, response::{http_response_with_child, http_response_with_err}, arguments::Argument, mylog::create_log_file, command};

pub fn execute_script(script: &str, stdout_log: &str, arguments: &Vec<String>) -> io::Result<Child>{
    
    let stdout_file = create_log_file(stdout_log).unwrap();
    let stderr_file = create_log_file(format!("{}.wf", stdout_log).as_str()).unwrap();
    let stdout = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);

    Command::new(script).args(arguments)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
}

pub fn is_valid_command(command: &str, work_dir: &str) -> std::io::Result<bool>{
    let command_full_path: String;
    if command.starts_with("./"){
        let command = command.trim_start_matches("./");
        let dir = work_dir.trim_end_matches('/');
        command_full_path = format!("{}/{}", dir, command);
    }else{
        command_full_path = command.to_string();
    }

    let status = Command::new("sh")
    .arg("-c")
    .arg(format!("command -v {}", command_full_path))//.arg(command_full_path.as_str())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .expect(format!("failed to execute process: {}", command_full_path).as_str());
    
    Ok(status.success())
}

pub fn trigger_hook(config: &Config, http_request: &HashMap<String, String>) -> String{
    // find the right config from config file for the incoming request
    let script = format!("{}/{}", config.command_working_directory, config.execute_command); 
    let arguments: Vec<String> = config.pass_arguments_to_command
    .iter()
    .map(|arg| Argument::new(arg)
                                        .unwrap()
                                        .parse_from_request(http_request)
                                        .unwrap())
    .collect();

    let stdout_log = config.get_log_path();
    let response = match execute_script(&script, &stdout_log, &arguments){
        Ok(c) => http_response_with_child(&c, http_request, config),
        Err(e) => http_response_with_err(&e, http_request, Some(config)),
    };
    response
}

#[test]
fn test_execute_script(){
    let args = vec!["-a".to_string(), "-l".to_string()];
    let log = format!("{}/logs/test.log", env!("CARGO_MANIFEST_DIR"));
    let _ = execute_script("ls", &log, &args);
    //process.try_wait();
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

// #[test]
// #[should_panic]
// fn test_isnot_valid_command(){
//     assert!(is_valid_command("ks", "/").unwrap())
// }

// #[test]
// fn test_is_valid_command(){
//     assert!(is_valid_command("ls", "/").unwrap())
// }

#[test]
fn test_is_valid_command_test_sh(){
    let work_dir = format!("{}/src/command/", env!("CARGO_MANIFEST_DIR"));
    let res = is_valid_command("./test.sh", &work_dir);
    assert!(res.unwrap())
}