use std::{collections::HashMap, io::{BufReader, BufRead, Read}, net::TcpStream};

pub fn parse_http_header(reader: &mut BufReader<&mut TcpStream>) -> HashMap<String, String>{
    let http_header: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_0: Vec<&str> = http_header[0].split(' ').collect();
    let mut header_map: HashMap<String, String> = HashMap::new();
    header_map.entry("Method".to_string()).or_insert(request_0[0].to_string());
    header_map.entry("Url".to_string()).or_insert(request_0[1].to_string());
    header_map.entry("Version".to_string()).or_insert(request_0[2].to_string());

    for item in http_header[1..].iter(){
        let request: Vec<&str>= item.split(": ").collect();
        let key = request[0].to_string();
        let value = request[1..].join(": ").to_string();
        header_map.entry(key).or_insert(value);
    }
    log::info!("{:#?}", header_map );
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

pub fn merge_http_request(header: &HashMap<String, String>, body: &Option<String>) -> HashMap<String, String>{
    match body {
        None => header.clone(),
        Some(s) => {
            let mut request:HashMap<String, String> = HashMap::new();
            request.clone_from(header);
            request.entry("Body".to_string()).or_insert(s.to_string());
            request 
        }
    }
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


#[test]
fn test_get_hook_id_from_http_request_url(){
    let id = parse_hook_id_from_url("/hook/?y=1&u=2");
    println!("{}", id);
}

#[test]
fn test_parse_parameters_from_url(){
    let url = "/hooks/?x=1&y=2&z=aaa";
    let res = parse_parameters_from_url(url);
    println!("{:#?}", res);
}