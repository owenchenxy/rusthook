use std::{process::{Command, Child, Stdio, ExitStatus}, str, fs::File, collections::HashMap, env, io};

use crate::{config::{configs::Configs, Config}, response::{http_response_with_child, http_response_with_err}};

pub fn execute_script(script: &str, stdout_log: &str) -> io::Result<Child>{
    let stdout_file = File::create(stdout_log).unwrap();
    let stderr_file = File::create(format!("{}.wf", stdout_log)).unwrap();
    let stdout = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);

    Command::new(script)
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
    // find the right config from config file for the incoming request
    let script = format!("{}/{}", config.command_working_directory, config.execute_command); 
    let stdout_log = config.get_log_path();
    let response = match execute_script(&script, &stdout_log){
        Ok(c) => http_response_with_child(&c, http_request, config),
        Err(e) => http_response_with_err(&e, http_request, Some(config)),
    };
    response
}

#[test]
fn test_execute_script(){
    let _ = execute_script("src/command/test.shs", "src/command/log/test.log");
    //process.try_wait();
}

#[test]
fn test_trigger_hook(){
    let configs: Configs = Configs::new("src/config/hooks.test.yaml");
    let mut http_request: HashMap<String, String> = HashMap::new();
    let config = configs.get_config_by_http_request(&http_request);

    http_request.insert("Id".to_string(), "webhook-test-1".to_string());
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