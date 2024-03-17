use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub host: String,
    pub headers: HashMap<String, String>,
}

#[derive(Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
}

impl StatusCode {
    pub fn get_status_text(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "200"),
            StatusCode::BadRequest => write!(f, "400"),
        }
    }
}

pub struct HttpResponse {
    pub status_code: StatusCode,
    pub protocol: String,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub content: String,
}

impl HttpResponse {
    pub fn get_status_line(&self) -> String {
        format!(
            "{} {} {}",
            self.protocol, self.status_code, self.status_text
        )
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let status_line = self.get_status_line();
        let content_length = format!("Content-Length: {}", self.content.len());

        format!(
            "{}\r\n{}\r\n\r\n{}",
            status_line, content_length, self.content
        )
        .into_bytes()
    }
}

pub fn parse_request(request: &Vec<String>) -> Result<HttpRequest, ()> {
    if let [request_line, host_line, raw_headers @ ..] = request.as_slice() {
        let request_line_parts = request_line.split_whitespace().collect::<Vec<&str>>();
        let host_line_parts = host_line.split_whitespace().collect::<Vec<&str>>();
        let raw_headers = &raw_headers[0];
        let headers = parse_headers(raw_headers);

        let http_request = HttpRequest {
            method: request_line_parts[0].to_string(),
            path: request_line_parts[1].to_string(),
            protocol: request_line_parts[2].to_string(),
            host: host_line_parts[1].to_string(),
            headers,
        };

        // println!("http request: {:#?}", http_request);

        Ok(http_request)
    } else {
        eprintln!("Failed to parse request");
        Err(())
    }
}

pub fn parse_headers(raw_headers: &String) -> HashMap<String, String> {
    let header_lines = raw_headers.lines();
    let mut header_map = HashMap::<String, String>::new();

    let _: Vec<_> = header_lines
        .map(|line| line.split_once(": ").unwrap())
        .inspect(|(key, value)| {
            header_map.insert(key.to_string(), value.to_string());
        })
        .collect();

    header_map
}
