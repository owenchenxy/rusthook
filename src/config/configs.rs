use std::sync::Arc;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap};
use std::env;

use crate::parser::parse_hook_id_from_url;

use super::{Config, global::GlobalConfig};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configs{
    pub global: GlobalConfig,
    pub hooks: Vec<Config>,
}

lazy_static! {
    pub static ref CONFIGS: Arc<Configs> = Arc::new(
        Configs::new(env::var("CONFIG_PATH").unwrap().as_str())
    );
}

impl Configs {
    pub fn new(config_file: &str) -> Configs{
        let configs_str = match fs::read_to_string(config_file){
            Ok(s) => s,
            Err(e) => {
                panic!("Cannot read file: {}, {}", config_file, e);
            }
        };
        serde_yaml::from_str(configs_str.as_str()).unwrap_or_else(|_| panic!("Cannot parse configs from config file: [{}]", config_file))
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
    let config_file = format!("{}/src/tests/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = Configs::new(&config_file);
    println!("{:#?}", configs);
}

#[test]
fn test_get_webhook_ids(){
    let config_file = format!("{}/src/tests/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = Configs::new(&config_file);
    let exp = vec![
        "webhook-test-1".to_string(),
        "webhook-test-2".to_string()
    ];
    let res = configs.get_webhook_ids();
    assert_eq!(res, exp);
}

#[test]
fn test_get_config_by_http_request(){
    let config_file = format!("{}/src/tests/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = Configs::new(&config_file);
    let mut http_request: HashMap<String, String> = HashMap::new();
    http_request.insert("Url".to_string(), "/webhook-test-2/".to_string());
    println!("{:#?}", configs.get_config_by_http_request(&http_request));
}

#[test]
fn test_global_config(){
    let config_file = format!("{}/src/tests/config/hooks.test.yaml", env!("CARGO_MANIFEST_DIR"));

}