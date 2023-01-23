use std::{collections::HashMap, hash::Hash, io};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use crate::config::configs;

use and_rule::*;
use not_rule::*;
use or_rule::*;
use single_rule::*;

pub mod and_rule;
pub mod or_rule;
pub mod not_rule;
pub mod single_rule;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Rule {
    Invalid,
    Single(SingleRule),
    Not(NotRule),
    And(AndRule),
    Or(OrRule),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AndStruct{
    pub and: Value
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OrStruct{
    pub or: Value
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NotStruct{
    pub not: Value
}

impl Rule{
    pub fn new(val: &serde_yaml::Value) -> Self{
        if let Ok(r) = serde_yaml::from_value::<AndStruct>(val.to_owned()){ 
            return AndRule::new(r.and)
        }
        if let Ok(r) = serde_yaml::from_value::<OrStruct>(val.to_owned()){ 
            return OrRule::new(r.or)
        }
        if let Ok(r) = serde_yaml::from_value::<NotStruct>(val.to_owned()){ 
            return NotRule::new(r.not)
        }
        if let Ok(r) = serde_yaml::from_value::<SingleRule>(val.to_owned()){ 
            return Rule::Single(r)
        }
        Rule::Invalid
    }

    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool{
        match self {
            Rule::Single(r) => r.is_matched(http_request),
            Rule::And(r) => r.is_matched(http_request),
            Rule::Or(r) => r.is_matched(http_request),
            Rule::Not(r) => r.is_matched(http_request),
            Rule::Invalid => false,
        }
    }
}
#[test]
fn test_parse_config_with_rule(){
    let config_file = format!("{}/src/config/hooks.test.rule.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = configs::Configs::new(&config_file);
    let rule = configs.hooks[0].trigger_rules.as_ref().unwrap();
    let rule = Rule::new(rule);
    println!("{:#?}", rule);
}