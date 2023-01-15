use std::collections::HashMap;

pub fn parse_http_request(http_request: &Vec<String>) -> HashMap<String, String>{
    println!();
    let request_0: Vec<&str> = http_request[0].split(' ').collect();
    let mut request_map: HashMap<String, String> = HashMap::new();
    request_map.entry("Method".to_string()).or_insert(request_0[0].to_string());
    request_map.entry("Id".to_string()).or_insert(request_0[1].to_string());
    request_map.entry("Version".to_string()).or_insert(request_0[2].to_string());

    for item in http_request[1..].iter(){
        let request: Vec<&str>= item.split(": ").collect();
        let key = request[0].to_string();
        let value = request[1..].join(": ").to_string();
        request_map.entry(key).or_insert(value);
    }
    request_map
}

#[test]
fn test_parse_http_request(){
    let mut http_request = Vec::new();
    http_request.push(String::from("GET / HTTP/1.1"));
    http_request.push(String::from("header: test_header"));
    let request = parse_http_request(&http_request);
    println!("{:#?}", request);
}