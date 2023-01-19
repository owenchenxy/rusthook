use std::{
    io::{prelude::*, BufReader, self}, net::TcpStream
};
pub mod config;
pub mod arguments;
pub mod check;
pub mod command;
pub mod parser;
pub mod response;
pub mod mylog;
use config::{configs::Configs, Config};
use check::*;
use command::*;
use parser::parse_http_header;
use response::http_response_with_err;
use arguments::*;

use crate::parser::{parse_http_body, merge_http_request};

pub fn handle_connection(mut stream: TcpStream, configs: Configs) -> Result<(), String>{   
    let mut reader = BufReader::new(&mut stream);
    // Get the http request header from tcpstream
    let http_header = parse_http_header(reader.by_ref());

    let mut body: Option<String> = None;
    // Get the http request body from the TcpStream for POST request
    if http_header.get("Method").unwrap() == "POST"{
        let content_length: usize = http_header.get("Content-Length").unwrap().parse().unwrap();
        body = parse_http_body(reader.by_ref(), content_length);
    }

    // build a completed http request map
    let http_request = merge_http_request(&http_header, &body);

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