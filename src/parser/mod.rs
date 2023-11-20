use std::{collections::HashMap, io::{BufReader, BufRead, Read}, net::{TcpStream, SocketAddr}};
use serde_json::Value;

pub fn parse_peer_address(reader: &mut BufReader<&mut TcpStream>) -> String{
    reader.get_ref().peer_addr().unwrap().to_string()
}

pub fn parse_http_header(reader: &mut BufReader<&mut TcpStream>) -> HashMap<String, String>{
    let http_header: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_0: Vec<&str> = http_header[0].split(' ').collect();
    let mut header_map: HashMap<String, String> = HashMap::from([
        ("Method".to_string(), request_0[0].to_string()),
        ("Url".to_string(), request_0[1].to_string()),
        ("Version".to_string(), request_0[2].to_string()),
    ]);

    for item in http_header[1..].iter(){
        let request: Vec<&str>= item.split(": ").collect();
        let key = request[0].to_string();
        let value = request[1..].join(": ").to_string();
        header_map.entry(key).or_insert(value);
    }
    header_map
}

pub fn parse_http_body(reader: &mut BufReader<&mut TcpStream>, content_length: usize) -> Option<String>{
    if content_length > 2097152{
        let err_msg = "maximum content-length(2M) exceeded";
        log::error!("{}", err_msg);
        panic!("{}", err_msg);
    }
    let mut buf = [0u8; 2097152];
    let mut res = reader.take(2097152);
    let _= res.read(&mut buf);
    let http_body = std::str::from_utf8(&mut buf[..content_length]).unwrap().to_string();
    Some(http_body)
}

pub fn merge_http_request(header: &HashMap<String, String>, body: &Option<String>, peer_addr: &SocketAddr) -> HashMap<String, String>{
    let mut request = match body {
        None => header.clone(),
        Some(s) => {
            let mut request:HashMap<String, String> = HashMap::new();
            request.clone_from(header);
            request.entry("Body".to_string()).or_insert(s.to_string());
            request 
        }
    };
    request.entry("Peer-Address".to_string()).or_insert(peer_addr.to_string());
    request
}

pub fn parse_hook_id_from_url(url: &str) -> String{
    let patterns: Vec<&str> = url.split("?").collect();
    patterns[0].trim_matches('/').to_string()
}

pub fn parse_parameters_from_url(url: &str) -> HashMap<String, String>{
    let patterns: Vec<&str> = url.split("?").collect();
    let parameters = patterns[1..].join("?").to_string();
    let mut params_map = HashMap::new();
    let params_vec: Vec<&str> = parameters.split('&').collect();
    for param in params_vec{
        let param_vec: Vec<&str> = param.split('=').collect();
        let key = param_vec[0].to_string();
        let val = param_vec[1].to_string();
        params_map.entry(key).or_insert(val);
    }
    params_map
}

pub fn get_item_from_json(v: &Value, item: &str) -> Option<String>{
    match &v[item] {
        Value::Null => {
            let mut indexes = item.split(".").into_iter();
            let mut val = &v.clone();
            loop{
                let next_index = indexes.next();
                if let Some(i) = next_index {
                    if let Ok(n) = i.parse::<usize>(){
                        val = &val[n];
                    }else{
                        val = &val[i];
                    }
                } else {
                    break Some(val.as_str().unwrap().to_string())
                }
            }  
        }
        Value::String(_) => Some(v[item].as_str().unwrap().to_string()),
        _ => {
            let err_msg = format!("Invalid parameter {} from payload", item);
            log::warn!("{}", err_msg);
            None
        },
    }
}

pub fn get_payload_item_from_http_request(item: &str, http_request: &HashMap<String, String>) -> Option<String>{
    match http_request.get("Body"){
        Some(payload) =>  {
            if item == "entire-payload" {
                Some(payload.to_string())
            }else{
                let v: Value = serde_json::from_str(payload).unwrap();
                get_item_from_json(&v, item)
            }
        },
        None => None
    }
}

pub fn get_header_from_http_request(name: &str, http_request: &HashMap<String, String>) -> Option<String>{
    http_request.get(name).cloned()
}

#[test]
pub fn test_json_parse(){
    let item = "data.data2";
    let mut indexes = item.split(".").into_iter();
    println!("{}", indexes.next().unwrap());
}

#[test]
pub fn test_get_item_in_map(){
    let v: Value = serde_json::from_str("{\"data\":{\"data2\":\"val\"}}").unwrap();
    let d = get_item_from_json(&v, "data.data2").unwrap();
    assert_eq!(String::from("val"), d);
}

#[test]
pub fn test_get_item_in_list(){
    let v: Value = serde_json::from_str("{\"data\":{\"data2\":[\"val1\", \"val2\"]}}").unwrap();
    let d = get_item_from_json(&v, "data.data2.1").unwrap();
    assert_eq!(String::from("val2"), d);
}

#[test]  
pub fn test_get_item_direct(){
    let v: Value = serde_json::from_str("{\"data\":{\"data2\":[\"val1\", \"val2\"]}, \"data_s\":\"s_d\"}").unwrap();
    let d = get_item_from_json(&v, "data_s").unwrap();
    assert_eq!(String::from("s_d"), d);
}

#[test]
fn test_get_hook_id_from_http_request_url(){
    let id = parse_hook_id_from_url("/hook/?y=1&u=2");
    assert_eq!(String::from("hook"), id);
}

#[test]
fn test_parse_parameters_from_url(){
    let url = "/hooks/?x=1&y=2&z=aaa";
    let res = parse_parameters_from_url(url);
    let exp = HashMap::from([
        ("x".to_string(), "1".to_string()),
        ("y".to_string(), "2".to_string()),
        ("z".to_string(), "aaa".to_string()),
    ]);
    assert_eq!(res, exp);
}
