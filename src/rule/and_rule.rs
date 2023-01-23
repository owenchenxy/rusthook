use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use serde_yaml::Value::Sequence;
use super::Rule;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AndRule{
    pub and: Vec<Rule>,
}

impl AndRule {
    pub fn new(conditions: Value) -> Rule{
        let and = conditions
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v|Rule::new(v))
            .filter(|r| {if let Rule::Invalid = r {false} else {true}})
            .collect(); 

        Rule::And(AndRule{and})
    } 

    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool{
        self.and.iter().all(|r|r.is_matched(http_request))
    }
}

#[test]
fn test_and_rule(){
    let config_file = format!("{}/src/config/hooks.test.rule.and.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = crate::config::configs::Configs::new(&config_file);
    let rule = configs.hooks[0].trigger_rules.as_ref().unwrap();
    let rule = Rule::new(rule);
    println!("{:#?}", rule);
}