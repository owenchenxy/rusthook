use log::{LevelFilter};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, env};

use crate::rule::Rule;

use self::global::GlobalConfig;
pub mod configs;
pub mod global;

type RespondHeader = HashMap<String, String>;
type CommandArgument = HashMap<String, String>;
type TriggerRules = serde_yaml::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "Config::default_id")]
    pub id: String,

    #[serde(default = "Config::default_execute_command")]
    pub execute_command: String,

    #[serde(default = "Config::default_command_working_directory")]
    pub command_working_directory: String,

    #[serde(default = "Config::default_response_message")]
    pub response_message: String,

    #[serde(default = "Config::default_response_headers")]
    pub response_headers: Vec<RespondHeader>,

    #[serde(default = "Config::default_pass_arguments_to_command")]
    pub pass_arguments_to_command: Vec<CommandArgument>,

    #[serde(default = "Config::default_log_dir")]
    pub log_dir: String,

    #[serde(default = "Config::default_log_prefix")]
    pub log_prefix: Option<String>,

    #[serde(default = "Config::default_log_level")]
    pub log_level: String,

    #[serde(default = "Config::default_trigger_rules")]
    pub trigger_rules: Option<TriggerRules>
}


impl Config {
    pub fn new() -> Self{
        Config{
            id: String::from("default"),
            execute_command: String::from(""),
            command_working_directory: String::from("."),
            response_message: String::from(""),
            response_headers: Vec::new(),
            pass_arguments_to_command: Vec::new(),
            log_dir: String::from("."),
            log_prefix: None,
            log_level: String::from("Info"),
            trigger_rules: None,
        }
    }
    
    pub fn get_log_path(&self) -> String{
        let log_prefix = match &self.log_prefix{
            Some(p) => &p,
            None => &self.id,
        };
        let log_dir = &self.log_dir.trim_end_matches("/").to_string();
        let stdout_log_path = format!("{}/{}.log", log_dir.as_str(), log_prefix.as_str());
        stdout_log_path.to_string()
    }

    pub fn get_log_level(&self) -> std::result::Result<LevelFilter, String>{
        match self.log_level.as_str() {
            "Off" => Ok(LevelFilter::Off),
            "Info" => Ok(LevelFilter::Info),
            "Warn" => Ok(LevelFilter::Warn),
            "Error" => Ok(LevelFilter::Error),
            "Debug" => Ok(LevelFilter::Debug),
            "Trace" => Ok(LevelFilter::Trace),
            _ => {
                Err(format!("Invalid Log level in config {:#?}", &self).to_string())
            }
        }
    }

    pub fn default_id() -> String{
        String::from("default")
    }

    pub fn default_execute_command() -> String{
        String::from("src/command/test.sh")
    }

    pub fn default_log_dir() -> String{
        GlobalConfig::default_log_dir()
    }

    pub fn default_log_prefix() -> Option<String>{
        GlobalConfig::default_log_prefix()
    }

    pub fn default_log_level() -> String{
        GlobalConfig::default_log_level()
    }

    pub fn default_command_working_directory() -> String{
        env::current_dir().unwrap().to_string_lossy().to_string()
    }
    
    pub fn default_response_message() -> String{
        String::new()
    }
    
    pub fn default_response_headers() -> Vec<HashMap<String, String>>{
        Vec::new()
    }

    pub fn default_pass_arguments_to_command() -> Vec<HashMap<String, String>>{
        Vec::new()
    }

    pub fn default_trigger_rules() -> Option<TriggerRules>{
        None
    }

    pub fn get_trigger_rule(&self) -> Option<Rule>{
        match &self.trigger_rules{
            None => None,
            Some(v) => Some(Rule::new(v))
        }
    }
}

