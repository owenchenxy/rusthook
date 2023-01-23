use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::Value;

use super::{Rule, single_rule::SingleRule};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NotRule{
    pub not: Box<Rule>,
}

impl NotRule {
    pub fn new(val: Value) -> Rule{
        let not = Rule::new(&val);
        Rule::Not(NotRule{not: Box::new(not)})
    }

    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool {
        !self.not.is_matched(http_request)
    }
}

#[test]
fn test_and_rule(){
    let config_file = format!("{}/src/config/hooks.test.rule.not.yaml", env!("CARGO_MANIFEST_DIR"));
    let configs = crate::config::configs::Configs::new(&config_file);
    let rule = configs.hooks[0].trigger_rules.as_ref().unwrap();
    let rule = Rule::new(rule);
    println!("{:#?}", rule);
}

