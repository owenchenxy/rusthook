use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::Value;

use super::Rule;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OrRule{
    pub or: Vec<Rule>,
}

impl OrRule {
    pub fn new(conditions: Value) -> Rule{
        let or = conditions
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v|Rule::new(v))
            .filter(|r| {if let Rule::Invalid = r {false} else {true}})
            .collect(); 

        Rule::Or(OrRule{or})
    } 

    pub fn is_matched(&self, http_request: &HashMap<String, String>) -> bool {
        self.or.iter().any(|r|r.is_matched(http_request))
    }
}

