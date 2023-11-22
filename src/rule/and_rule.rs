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
