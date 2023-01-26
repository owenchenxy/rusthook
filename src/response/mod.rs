use std::{io::{self, Write}, process::Child, collections::HashMap, net::TcpStream, fs};
use serde_json::json;

use crate::config::Config;
use favicon::FAVICON;
pub mod favicon;

pub fn format_response_headers_to_string(headers: &Vec<HashMap<String, String>>) -> String{
    headers.iter()
    .map(| item | {
        let name = item.get("name").unwrap().as_str();
        let value = item.get("value").unwrap().as_str();
        vec![name, value].join(": ")
    })
    .collect::<Vec<String>>()
    .join("\r\n")
}

pub fn http_response_with_child(stream: &mut TcpStream, child: &Child, http_request: &HashMap<String, String>, config: &Config) {
    let status_line = format!("{} 200 OK", http_request.get("Version").unwrap());
    let config_str = serde_json::to_string(&config).unwrap();
    let headers = format_response_headers_to_string(&config.response_headers);
    let contents = json!({
        "message": config.response_message,
        "config": format!("{}", config_str),
        "thread": child.id(),
    }).to_string();
    let length = contents.len();
    let response = format!("{status_line}\r\n\
    Content-Length: {length}\r\n\
    {headers}\r\n\
    \r\n\
    {contents}");
    stream.write_all(response.as_bytes()).unwrap()
}

pub fn http_response_with_err(stream: &mut TcpStream, err: &io::Error, http_request: &HashMap<String, String>, config: Option<&Config>) {
    let status_line: String;
    match err.kind() {
        io::ErrorKind::NotFound => {
            status_line = format!("{} 404 Not Found", http_request.get("Version").unwrap());
        },
        _ => {
            status_line = format!("{} 500 Internal Server Error", http_request.get("Version").unwrap());
        }
    }
    let err_msg = err.to_string();
    let contents = match config{
        None => &err_msg,
        Some(_) => &status_line,
    };

    let length = contents.len();
    let response = format!("{status_line}\r\n\
    Content-Length: {length}\r\n\
    \r\n\
    {contents}");
    stream.write_all(response.as_bytes()).unwrap()
}

pub fn respond_with_favicon(stream: &mut TcpStream){
    let response = format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type:image/x-icon\r\n\
    \r\n", FAVICON.len());  
    stream.write(response.as_bytes()).unwrap();
    stream.write_all(FAVICON).unwrap();
    stream.flush().unwrap();
}

#[test]
pub fn test_format_headers_to_string(){
    let header_host = HashMap::from([
        ("name".to_string(), "Host".to_string()),
        ("value".to_string(), "127.0.0.1:7878".to_string()),
    ]);
    let header_cache_control = HashMap::from([
        ("name".to_string(), "Cache-Control".to_string()),
        ("value".to_string(), "max-age=604800".to_string()),
    ]);
    let header_accept_ranges = HashMap::from([
        ("name".to_string(), "Accept-Ranges".to_string()),
        ("value".to_string(), "bytes".to_string()),
    ]);

    let headers = vec![header_host, header_accept_ranges, header_cache_control];
    let headers = format_response_headers_to_string(&headers);
    let exp = "Host: 127.0.0.1:7878\r\n\
    Accept-Ranges: bytes\r\n\
    Cache-Control: max-age=604800".to_string();
    assert_eq!(exp, headers);
}