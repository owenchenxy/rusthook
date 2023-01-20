use std::{process::{Command, Child, Stdio, ExitStatus}, str, fs::File, collections::HashMap, env, io};

use crate::{config::{configs::Configs, Config}, response::{http_response_with_child, http_response_with_err}, arguments::Argument};

pub fn execute_script(script: &str, stdout_log: &str, arguments: &Vec<String>) -> io::Result<Child>{
    let stdout_file = File::create(stdout_log).unwrap();
    let stderr_file = File::create(format!("{}.wf", stdout_log)).unwrap();
    let stdout = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);

    Command::new(script).args(arguments)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
}

pub fn is_valid_command(command: &str, work_dir: &str) -> std::io::Result<bool>{
    let cwd = env::current_dir()?;
    let _ = env::set_current_dir(work_dir);   
     
    let status = Command::new("command")
    .arg("-v").arg(command)
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .expect("failed to execute process");

    let _ = env::set_current_dir(cwd); 
    Ok(status.success())
}

pub fn trigger_hook(config: &Config, http_request: &HashMap<String, String>) -> String{
    let cwd = env::current_dir();
    println!("{:#?}", cwd);    // find the right config from config file for the incoming request
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
    let _ = execute_script("ls", "test.log", &args);
    //process.try_wait();
}

#[test]
fn test_trigger_hook(){
    let configs: Configs = Configs::new("src/config/hooks.test.yaml");
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Method".to_string(), "GET".to_string());
    http_request.insert("Url".to_string(), "/webhook-test-1/".to_string());
    http_request.insert("Host".to_string(), "127.0.0.1:7878".to_string());
    http_request.insert("Version".to_string(), "HTTP/1.1".to_string());
    let config = configs.get_config_by_http_request(&http_request);
    trigger_hook(&config, &http_request);
    ()
}

#[test]
#[should_panic]
fn test_is_valid_command(){
    assert!(is_valid_command("ks", "/").unwrap())
}

#[test]
fn test_is_valid_command_test_sh(){
    assert!(is_valid_command("./test.sh", "src/command/").unwrap())
}