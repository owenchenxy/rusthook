use std::{collections::HashMap, vec, net::{IpAddr, Ipv4Addr}, error::Error, fs};

use ipnet::IpNet;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use crypto::{sha1::Sha1, hmac::Hmac, mac::Mac, sha2::{Sha256, Sha512}};

use crate::{parser::*, config::configs::CONFIGS};
use regex::Regex;

use super::Rule;


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SingleRule {
    pub kind:  String,
    pub value: String,

    #[serde(default = "SingleRule::default_source")]
    pub source: String,

    #[serde(default = "SingleRule::default_name")]
    pub name: String,
}

impl SingleRule {
    pub fn default_source() -> String{
        String::new()
    }

    pub fn default_name() -> String{
        String::new()
    }

    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool {
        match self.kind.as_str(){
            "value" => self.match_value(http_request),
            "regex" => self.match_regex(http_request),
            "hmac-sha1" => self.match_hmac_sha1(http_request),
            "hmac-sha256" => self.match_hmac_sha256(http_request),
            "hmac-sha512" => self.match_hmac_sha512(http_request),
            "ip-whitelist" => self.match_ip_whitelist(http_request),
            "include" => {
                // read rule from the file specified by value
                match get_rule_from_file(&self.value){
                    Ok(r) => r.is_matched(http_request),
                    Err(_) => false,
                }
            },
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
                let exp = self.value.to_string();
                let re = match Regex::new(&exp){
                    Ok(r) => r,
                    Err(e) => {
                        let msg = format!("Invalid Regex {}, {}", exp, e);
                        log::error!("{}", msg);
                        return false
                    }
                };                
                re.is_match(r.as_str())
            },
            None => false
        }  
    }

    fn match_hmac_sha1(&self, http_request: &HashMap<String, String>) -> bool {
        let payload = http_request.get("Body");
        let secret = &self.value;
        let signature = match get_header_from_http_request(self.name.as_str(), http_request){
            None => {
                let msg = format!("Header [{}] not found in request headers", self.name);
                log::warn!("{}", msg);
                return false
            },
            Some(s) => s,
        };
        check_payload_signature(payload, secret, signature.as_str())
    }

    fn match_hmac_sha256(&self, http_request: &HashMap<String, String>) -> bool {
        let payload = http_request.get("Body");
        let secret = &self.value;
        let signature = match get_header_from_http_request(self.name.as_str(), http_request){
            None => {
                let msg = format!("Header [{}] not found in request headers", self.name);
                log::warn!("{}", msg);
                return false
            },
            Some(s) => s,
        };
        check_payload_signature256(payload, secret, signature.as_str())
    }

    fn match_hmac_sha512(&self, http_request: &HashMap<String, String>) -> bool {
        let payload = http_request.get("Body");
        let secret = &self.value;
        let signature = match get_header_from_http_request(self.name.as_str(), http_request){
            None => {
                let msg = format!("Header [{}] not found in request headers", self.name);
                log::warn!("{}", msg);
                return false
            },
            Some(s) => s,
        };
        check_payload_signature512(payload, secret, signature.as_str())    
    }

    fn match_ip_whitelist(&self, http_request: &HashMap<String, String>) -> bool {
        let peer_address = http_request.get("Peer-Address").unwrap();
        let ip = match peer_address.split(':').collect::<Vec<&str>>()[0].parse::<IpAddr>(){
            Ok(r) => r,
            Err(e) => {
                let msg = format!("Invalid Peer Address {}: {}", peer_address, e);
                log::warn!("{}", msg);
                return false;
            }
        };

        let ip_ranges: Vec<String>;
        if self.value.contains(','){
            ip_ranges = extract_comma_separated_values(&self.value, "");
        }else{
            ip_ranges = vec![(*self.value).to_string()];
        }

        ip_ranges
        .iter()
        .map(|cidr|{
            match cidr.parse::<IpNet>(){
                Ok(r) => r,
                Err(e) => {
                    let msg = format!("Invalid IP CIDR {}: {}", cidr, e);
                    log::error!("{}", msg);
                    IpNet::new(Ipv4Addr::new(0, 0, 0, 0).into(), 32).unwrap()
                }
            }
        })
        .collect::<Vec<IpNet>>()
        .iter()
        .any(|x|x.contains(&ip))        
    }
}

fn get_rule_from_file(rule_file: &str) -> Result<Rule, Box<dyn Error>> {
    let rule_str = match fs::read(format!("{}/{}", CONFIGS.global.rules_dir, &rule_file)) {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("Failed to read rule from file {}/{}: {}", CONFIGS.global.rules_dir, &rule_file, e);
            log::error!("{}", msg);
            return Err(Box::new(e));
        }
    };

    let rule = match String::from_utf8(rule_str){
        Ok(r) => Rule::new(&serde_yaml::from_str(r.as_str()).unwrap()),
        Err(e) => {
            let msg = format!("Failed to convert rule from file {}/{} to string: {}", CONFIGS.global.rules_dir, rule_file, e);
            log::error!("{}", msg);
            return Err(Box::new(e));
        }
    };
    Ok(rule)
}

fn check_payload_signature(payload: Option<&String>, secret: &str, signature: &str)-> bool{
    if secret.is_empty() {
        let err_msg = "signature validation secret can not be empty".to_string();
        log::error!("{}", err_msg);
        return false;
	}
	// Extract the signatures.
	let signatures = extract_signatures(signature, "sha1=");
	// Validate the MAC.
	validate_mac(payload, &mut Hmac::new(Sha1::new(), secret.as_bytes()), signatures)
}

fn check_payload_signature256(payload: Option<&String>, secret: &str, signature: &str)-> bool{
    if secret.is_empty() {
        let err_msg = "signature validation secret can not be empty".to_string();
        log::error!("{}", err_msg);
        return false;
	}
	// Extract the signatures.
	let signatures = extract_signatures(signature, "sha256=");
	// Validate the MAC.
	validate_mac(payload, &mut Hmac::new(Sha256::new(), secret.as_bytes()), signatures)
}

fn check_payload_signature512(payload: Option<&String>, secret: &str, signature: &str)-> bool{
    if secret.is_empty() {
        let err_msg = "signature validation secret can not be empty".to_string();
        log::error!("{}", err_msg);
        return false;
	}
	// Extract the signatures.
	let signatures = extract_signatures(signature, "sha512=");
	// Validate the MAC.
	validate_mac(payload, &mut Hmac::new(Sha512::new(), secret.as_bytes()), signatures)
}

fn extract_signatures(source: &str, prefix: &str) -> Vec<String> {
    if source.contains(','){
        return extract_comma_separated_values(source, prefix);
    }
    vec![source.trim_start_matches(prefix).to_string()]
}

fn extract_comma_separated_values(source: &str, prefix: &str) -> Vec<String>{
    let values: Vec<String> = source.split(',')
        .map(|s|s.trim())
        .filter(|s|s.starts_with(prefix))
        .map(|s|s.trim_start_matches(prefix).to_string())
        .collect();
    values
}

fn validate_mac<T>(payload: Option<&String>, hasher: &mut T, signatures: Vec<String>) -> bool
where T: Mac{
    if payload.is_none(){
        let msg = "HMAC validation failed due to empty payload !".to_string();
        log::warn!("{}", msg);
        return false;
    }
    hasher.input(payload.unwrap().as_bytes());
    let actual_mac = hasher.result();
    let expected_signature = actual_mac
                                    .code()
                                    .iter()
                                    .format_with("", |byte, f| f(&format_args!("{:02x}", byte)))
                                    .to_string();

    signatures.iter()
              .map(|s| s.eq(&expected_signature))
              .collect::<Vec<bool>>()
              .iter()
              .any(|x| *x)
}

#[test]
fn test_match_hmac_sha1(){
    let mut request: HashMap<String, String> = HashMap::from([
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
        ("X-Signature".to_string(), "".to_string()),
    ]);

    let single_rule = SingleRule{
        kind: "hmac-sha1".to_string(),
        value: "1234".to_string(),
        source: "header".to_string(),
        name: "X-Signature".to_string(),
    };

    let mut hasher = Hmac::new(Sha1::new(), single_rule.value.as_bytes());
    hasher.input(request.get("Body").unwrap().as_bytes());
    let actual_mac = hasher.result();
    let actual_mac = actual_mac
        .code()
        .iter()
        .format_with("", |byte, f| f(&format_args!("{:02x}", byte)))
        .to_string();
    let sig = format!("sha1=dcwwcwee, sha1={}", actual_mac);
    request.insert("X-Signature".to_string(), sig.to_string());
    assert!(single_rule.match_hmac_sha1(&request));
}

#[test]
fn test_extract_signature(){
    let source = "sha1=8888";
    let prefix = "sha1=";
    assert_eq!(vec!["8888"], extract_signatures(source, prefix));
    let source = "sha1=11111, sha1=22222, sha1=33333, sha256=44444, sha1=55555";
    assert_eq!(vec!["11111","22222","33333","55555"], extract_signatures(source, prefix));
}

#[test]
fn test_extract_comma_separated_values(){
    let source = "sha1=11111, sha1=22222, sha1=33333, sha256=44444, sha1=55555";
    let prefix = "sha1=";
    assert_eq!(vec!["11111","22222","33333","55555"], extract_comma_separated_values(source, prefix))
}

#[test]
fn test_match_ip_white_list(){
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
        ("Peer-Address".to_string(), "10.0.2.6:56020".to_string()),
    ]);

    let single_rule = SingleRule{
        kind: "ip-whitelist".to_string(),
        value: "10.0.1.2/24, 10.0.2.5/24, 10.0.0.1/32".to_string(),
        source: "".to_string(),
        name: "".to_string(),
    };
    assert!(single_rule.match_ip_whitelist(&request));
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

#[test]
fn test_extract_comma_separated_cidr(){
    let s = "1.1.1.1/24,2.2.2.2/24";
    println!("{:#?}", extract_comma_separated_values(s, ""));
}

#[test]
fn test_read_rule_from_file(){
    let rule_file = "src/tests/rule/rule.test.yaml";
    let rule = get_rule_from_file(rule_file);
    println!("{:#?}", rule);
}

#[test]
fn test_include_rules(){
    let config_file = format!("{}/src/tests/config/hooks.test.rule.include.yaml", env!("CARGO_MANIFEST_DIR"));
    use std::env;
    env::set_var("CONFIG_PATH", &config_file);
    let rule = CONFIGS.hooks[0].trigger_rules.as_ref().unwrap();
    let rule = Rule::new(rule);

    let http_request = HashMap::from([
        ("Url".to_string(), "/webhook-test-1".to_string()),
        ("User-Agent".to_string(), "curl/7.77.0".to_string()),
        ("Version".to_string(), "HTTP/1.1".to_string()),
        ("Method".to_string(), "POST".to_string()),
        ("Accept".to_string(), "*/*".to_string()),
        ("Host".to_string(), "127.0.0.1:7878".to_string())
        ]);

    assert!(rule.is_matched(&http_request));
}