use std::{collections::HashMap, io};

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::parser::{parse_parameters_from_url, get_item_from_json};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub source: String,
    pub name:   String,
}

impl Argument {
    pub fn new(builder: &HashMap<String, String>) -> Option<Self>{
        if builder.len()!= 2 || !builder.contains_key("source") || !builder.contains_key("name"){
            return None;
        }
        let source = builder.get("source").unwrap().to_string();
        let name= builder.get("name").unwrap().to_string();
        Some(Argument { source, name })
    }

    fn get_argument_from_map(&self, map: &HashMap<String, String>, name: &String) -> Result<String, io::Error>{
        let key = &self.name;
        match map.get(key){ 
            Some(v) => Ok(v.to_string()),
            None => {
                let err_msg = format!("Failed to get parameter [{}]", &name);
                log::error!("{}", err_msg);

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    err_msg,
                ))
            }  
        }
    }

    pub fn get_argument_from_header(&self, request: &HashMap<String, String>, name: &String) -> Result<String, io::Error>{
        let invalid_headers = [
            String::from("Method"), 
            String::from("Url"), 
            String::from("Version"), 
            String::from("Body"),
            ];
        if invalid_headers.contains(&name){
            let err_msg = format!("Invalid Header Name [{}]", name);
            log::error!("{}", err_msg);

            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                err_msg,
            ))
        }
        self.get_argument_from_map(request, name)
    }

    pub fn get_argument_from_query(&self, request: &HashMap<String, String>, name: &String) -> Result<String, io::Error>{
        let url = request.get("Url").unwrap();
        let params = parse_parameters_from_url(url);
        self.get_argument_from_map(&params, name)
    }

    pub fn get_argument_from_payload(&self, request: &HashMap<String, String>, name: &String) -> Result<String, io::Error>{
        if request.get("Method").unwrap() == "GET" {
            let err_msg = format!("Could not parse argument [{}] from GET request with no payload", name);
            log::warn!("{}", err_msg);

            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                err_msg,
            ))
        }
        let payload = request.get("Body").unwrap();
        if name == "" {
            let err_msg = format!("Pass entire payload as argument");
            log::warn!("{}", err_msg);
            return Ok(payload.to_string());
        }
        let v: Value = serde_json::from_str(payload).unwrap();
        
        let err_msg = format!("Get argument [{}] from payload", name);
        log::warn!("{}", err_msg);
        match get_item_from_json(&v, name.as_str()){
            None => Ok(String::new()),
            Some(s) => Ok(s)
        }
    }

    pub fn get_argument_from_request(&self, request: &HashMap<String, String>, name: &String) -> Result<String, io::Error>{
        let valid_request_sources = [
            "method",
            "peer-address",
            "url",
            "version",
        ];
        if !valid_request_sources.contains(&name.to_lowercase().as_str()){
            let err_msg = format!("Invalid request parameter [{}]", name);
            log::error!("{}", err_msg);

            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                err_msg,
            ))
        }
        self.get_argument_from_map(request, name)
    }

    pub fn parse_from_request(&self, request: &HashMap<String, String>) -> Result<String, io::Error>{
        match self.source.as_str() {
            "string" => Ok(self.name.clone()),
            "payload" => self.get_argument_from_payload(request, &self.name),
            "query" => self.get_argument_from_query(request, &self.name),
            "header" => self.get_argument_from_header(request, &self.name),
            "request" => self.get_argument_from_request(request, &self.name),
            _ => {
                let err_msg = format!("Invalid request parameter [{}]", self.name);
                log::error!("{}", err_msg);

                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    err_msg,
                ))
            }
        }
    }
}


#[test]
pub fn test_parse_arg(){
    let request: HashMap<String, String> = HashMap::from([
        ("Url".to_string(), "/webhook-test-1".to_string()),
        ("User-Agent".to_string(), "curl/7.77.0".to_string()),
        ("Version".to_string(), "HTTP/1.1".to_string()),
        ("Method".to_string(), "POST".to_string()),
        ("Accept".to_string(), "*/*".to_string()),
        ("Host".to_string(), "127.0.0.1:7878".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Content-Length".to_string(), "45".to_string()),
        ("Body".to_string(), "{\"data\":{\"data2\":[\"val1\", \"val2\"], \"data3\": \"val3\"},\"data_s\":\"s_d\"}".to_string()),
        ("Peer-Address".to_string(), "127.0.0.1:56020".to_string()),
    ]);

    let map = HashMap::from([
        ("source".to_string(), "payload".to_string()),
        ("name".to_string(), "".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());

    let map = HashMap::from([
        ("source".to_string(), "request".to_string()),
        ("name".to_string(), "Method".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());
    
    let map = HashMap::from([
        ("source".to_string(), "payload".to_string()),
        ("name".to_string(), "data.data2.1".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());

    let map = HashMap::from([
        ("source".to_string(), "payload".to_string()),
        ("name".to_string(), "data.data3".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());

    let map = HashMap::from([
        ("source".to_string(), "payload".to_string()),
        ("name".to_string(), "data_s".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());

    let map = HashMap::from([
        ("source".to_string(), "header".to_string()),
        ("name".to_string(), "Host".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());

    let map = HashMap::from([
        ("source".to_string(), "string".to_string()),
        ("name".to_string(), "str_param".to_string()),
    ]);
    let arg = Argument::new(&map).unwrap();
    let res = arg.parse_from_request(&request).unwrap();
    println!("{}, {}", res, res.len());
}