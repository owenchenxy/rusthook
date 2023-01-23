use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::Value;

use crate::parser::*;
use regex::Regex;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SingleRule {
    pub kind:  String,
    pub value: String,
    pub source: String,
    pub name: String,
}
impl SingleRule {
    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool {
        match self.kind.as_str(){
            "value" => self.match_value(http_request),
            "regex" => self.match_regex(http_request),
            "hmac-sha1" => self.match_hmac_sha1(http_request),
            "hmac-sha256" => self.match_hmac_sha1(http_request),
            "hmac_sha512" => self.match_hmac_sha512(http_request),
            "ip_whitelist" => self.match_ip_whitelist(http_request),
            &_ => false,
        }
    }

    fn get_value_from_source(&self, http_request: &HashMap<String, String>) -> Option<String>{
        match self.source.as_str(){
            "header" => get_header_from_http_request(self.name.as_str(), http_request),
            "payload" => get_payload_item_from_http_request(&self.name, http_request),
            _ => None
        }
    }

    fn match_value(&self, http_request: &HashMap<String, String>) -> bool{
        match self.get_value_from_source(http_request) {
            Some(r) => r == self.value,
            None => false
        }  
    }

    fn match_regex(&self, http_request: &HashMap<String, String>) -> bool {
        match self.get_value_from_source(http_request) {
            Some(r) => {
                let exp = format!(r"{}", self.value);
                let re = match Regex::new(&exp){
                    Ok(r) => r,
                    Err(e) => {
                        let msg = format!("Invalid Regex {}, {}", exp, e.to_string());
                        log::error!("{}", msg);
                        return false
                    }
                };                
                re.is_match(r.as_str())
            },
            None => return false
        }  
    }

    fn match_hmac_sha1(&self, http_request: &HashMap<String, String>) -> bool {
        todo!()
    }

    fn match_hmac_sha512(&self, http_request: &HashMap<String, String>) -> bool {
        todo!()
    }

    fn match_ip_whitelist(&self, http_request: &HashMap<String, String>) -> bool {
        todo!()
    }
}


#[test]
fn test_match_value(){
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

    let single_rule = SingleRule{
        kind: "value".to_string(),
        value: "127.0.0.1:7878".to_string(),
        source: "header".to_string(),
        name: "Host".to_string(),
    };
    assert!(single_rule.match_value(&request));

    let single_rule = SingleRule{
        kind: "value".to_string(),
        value: "val1".to_string(),
        source: "payload".to_string(),
        name: "data.data2.0".to_string(),
    };
    assert!(single_rule.match_value(&request));
}

#[test]
fn test_match_regex(){
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

    let single_rule = SingleRule{
        kind: "regex".to_string(),
        value: "127.0.0.1:i*".to_string(),
        source: "header".to_string(),
        name: "Host".to_string(),
    };
    assert!(single_rule.match_regex(&request));

    let single_rule = SingleRule{
        kind: "regex".to_string(),
        value: "127.0.0.1:ii*".to_string(),
        source: "header".to_string(),
        name: "Host".to_string(),
    };
    assert!(!single_rule.match_regex(&request));
}
