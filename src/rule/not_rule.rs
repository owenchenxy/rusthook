use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use super::Rule;

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


