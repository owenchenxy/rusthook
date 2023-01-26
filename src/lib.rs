use std::{
    io::{prelude::*, BufReader}, net::TcpStream, fs
};
pub mod config;
pub mod arguments;
pub mod check;
pub mod command;
pub mod parser;
pub mod response;
pub mod mylog;
mod rule;
use config::{configs::Configs};
use check::*;
use command::*;
use parser::{parse_http_header, parse_hook_id_from_url};
use response::{http_response_with_err, respond_with_favicon};

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
    // Get the peer address from whom the request was sent
    let peer_addr = reader.by_ref().get_ref().peer_addr().unwrap();

    // build a completed http request map
    let http_request = merge_http_request(&http_header, &body, &peer_addr);

    if parse_hook_id_from_url(http_request.get("Url").unwrap()) == "favicon.ico"{
        respond_with_favicon(&mut stream);
        return Ok(());
    }
    // check if the id in request defined in configs 
    if let Err(e) = is_webhook_id_in_configs(&configs, &http_request) {
        http_response_with_err(&mut stream, &e, &http_request, None);
        return Ok(());
    };

    // get the right config
    let config = &configs.get_config_by_http_request(&http_request);

    // preflight check according to the found config
    if let Err(e) = preflight_check(&config, &http_request){
        http_response_with_err(&mut stream, &e, &http_request, None);
        return Ok(());
    };

    // generate response and send
    trigger_hook(&mut stream, &config, &http_request);
    Ok(())
}