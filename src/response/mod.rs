use std::{io, process::Child, collections::HashMap};

use serde_json::json;

use crate::config::Config;

pub fn http_response_with_child(child: &Child, http_request: &HashMap<String, String>, config: &Config) -> String{
    let status_line = format!("{} 200 OK", http_request.get("Version").unwrap());
    let config_str = serde_json::to_string(&config).unwrap();
    let contents = json!({
        "message": config.response_message,
        "config": format!("{}", config_str),
        "thread": child.id(),
    }).to_string();
    let length = contents.len();
    format!("{status_line}\r\n\
    Content-Length: {length}\r\n\
    \r\n\
    {contents}")
}

pub fn http_response_with_err(err: &io::Error, http_request: &HashMap<String, String>, config: Option<&Config>) -> String{
    let status_line: String;
    match err.kind() {
        io::ErrorKind::NotFound => {
            status_line = format!("{:#?} 404 Not Found", http_request.get("Version").unwrap());
        },
        _ => {
            status_line = format!("{:#?} 500 Internal Server Error", http_request.get("Version").unwrap());
        }
    }
    let err_msg = err.to_string();
    let contents = match config{
        None => &err_msg,
        Some(c) => &c.response_message,
    };

    let length = contents.len();
    format!("{status_line}\r\n\
    Content-Length: {length}\r\n\
    \r\n\
    {contents}")
}