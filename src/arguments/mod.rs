use std::{collections::HashMap, io};

use crate::parser::parse_parameters_from_url;

pub struct Argument {
    pub source: String,
    pub name:   String,
}

impl Argument {
    pub fn new() -> Option<Self>{
        None
    }

    fn get_argument_from_map(&self, map: &HashMap<String, String>, name: String) -> Result<String, io::Error>{
        let key = &self.name;
        match map.get(key){ 
            Some(v) => Ok(v.to_string()),
            None => {
                let err_msg = format!("Failed to get parameter [{}] from header", &name);
                log::error!("{}", err_msg);

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    err_msg,
                ))
            }  
        }
    }

    fn get_argument_from_header(&self, request: &HashMap<String, String>, name: String) -> Result<String, io::Error>{
        self.get_argument_from_map(request, name)
    }

    fn get_argument_from_query(&self, request: &HashMap<String, String>, name: String) -> Result<String, io::Error>{
        let url = request.get("Url").unwrap();
        let params = parse_parameters_from_url(url);
        self.get_argument_from_map(&params, name)
    }
}