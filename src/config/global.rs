use log::LevelFilter;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    #[serde(default = "GlobalConfig::default_log_dir")]
    pub log_dir: String,

    #[serde(default = "GlobalConfig::default_log_prefix")]
    pub log_prefix: Option<String>,

    #[serde(default = "GlobalConfig::default_log_level")]
    pub log_level: String,
}


impl GlobalConfig {
    pub fn new() -> Self{
        GlobalConfig { 
            log_dir: Self::default_log_dir(), 
            log_prefix: Self::default_log_prefix(), 
            log_level: Self::default_log_level() 
        }
    }

    pub fn default_log_dir() -> String{
        String::from("logs")
    }

    pub fn default_log_prefix() -> Option<String>{
        None
    }

    pub fn default_log_level() -> String{
        String::from("Info")
    }

    pub fn get_log_path(&self) -> String{
        let log_prefix = match &self.log_prefix{
            Some(p) => &p,
            None => "webhook",
        };
        let log_dir = &self.log_dir.trim_end_matches("/").to_string();
        let stdout_log_path = format!("{}/{}.log", log_dir.as_str(), log_prefix);
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
}