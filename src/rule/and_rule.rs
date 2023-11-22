use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
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
            .map(Rule::new)
            .filter(|r| {!matches!(r, Rule::Invalid)})
            .collect(); 

        Rule::And(AndRule{and})
    } 

    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool{
        self.and.iter().all(|r|r.is_matched(http_request))
    }
}

#[test]
fn test_and_rule(){
    let config_file = format!("{}/src/tests/config/hooks.test.rule.and.yaml", env!("CARGO_MANIFEST_DIR"));
    use std::env;
    use crate::config::configs::CONFIGS;
    env::set_var("CONFIG_PATH", config_file);
    let rule = CONFIGS.hooks[0].trigger_rules.as_ref().unwrap();
    let rule = Rule::new(rule);
    println!("{:#?}", rule);
}