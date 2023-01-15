use std::{
    io::{prelude::*, BufReader, self}, net::TcpStream
};
pub mod config;
pub mod check;
pub mod command;
pub mod parser;
pub mod response;
pub mod mylog;
use config::{configs::Configs, Config};
use check::*;
use command::*;
use parser::parse_http_request;
use response::http_response_with_err;

pub fn handle_connection(mut stream: TcpStream, configs: Configs) -> Result<(), String>{    
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = parse_http_request(&http_request);
    
    // check if the id in request defined in configs 
    if let Err(e) = is_webhook_id_in_configs(&configs, &http_request) {
        let response = http_response_with_err(&e, &http_request, None);
        stream.write_all(response.as_bytes()).unwrap();
        return Ok(());
    };

    // get the right config
    let config = &configs.get_config_by_http_request(&http_request);

    // preflight check according to the found config
    if let Err(e) = preflight_check(&config){
        let response = http_response_with_err(&e, &http_request, None);
        stream.write_all(response.as_bytes()).unwrap();
        return Ok(());
    };

    // generate response and send
    let response = trigger_hook(&config, &http_request);
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}