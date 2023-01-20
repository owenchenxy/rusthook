use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap};

use crate::parser::parse_hook_id_from_url;

use super::{Config, global::GlobalConfig};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configs{
    pub global: GlobalConfig,
    pub hooks: Vec<Config>,
}

impl Configs {
    pub fn new(config_file: &str) -> Configs{
        let configs_str = fs::read_to_string(config_file)
                                    .expect(format!("Cannot found file: {}", config_file).as_str());
        serde_yaml::from_str(configs_str.as_str()).expect(format!("Cannot parse configs from config file: [{}]", config_file).as_str())
    }

    pub fn get_webhook_ids(&self) -> Vec<String>{
        let mut webhook_ids = Vec::new();
        for item in self.hooks.iter(){
            webhook_ids.push(item.id.clone());
        }
        webhook_ids
    }

    pub fn get_config_by_http_request(&self, http_request: &HashMap<String, String>) -> Config{
        let mut config: Config = Config::new();
        let mut is_default = true;
        let url = http_request.get("Url").unwrap();
        let requested_id = parse_hook_id_from_url(url);
        for item in self.hooks.iter(){
            if item.id == requested_id{
                config = item.clone();
                is_default = false;
            }
        }
        if is_default{
            log::info!("Using default config: {:#?}", config);
        }
        config
    }

    pub fn get_global_log_path(&self) -> String{
        self.global.get_log_path()
    }
}

#[test]
fn test_parse_config_from_yaml(){
    let configs = Configs::new("src/config/hooks.test.yaml");
    println!("{:#?}", configs);
}

#[test]
fn test_get_webhook_ids(){
    let configs = Configs::new("src/config/hooks.test.yaml");
    let exp = vec![
        "webhook-test-1".to_string(),
        "webhook-test-2".to_string()
    ];
    let res = configs.get_webhook_ids();
    assert_eq!(res, exp);
}

#[test]
fn test_get_config_by_http_request(){
    let configs = Configs::new("src/config/hooks.test.yaml");
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Id".to_string(), "webhook-test-2".to_string());
    println!("{:#?}", configs.get_config_by_http_request(&http_request));
}