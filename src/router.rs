use crate::http::{HttpRequest, HttpResponse, StatusCode};
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::time::Duration;

pub fn handle_request(request: &HttpRequest) -> HttpResponse {
    let (status_code, content) = route(&request.path);
    HttpResponse {
        status_code,
        content,
        protocol: request.protocol.to_owned(),
        status_text: status_code.get_status_text().to_string(),
        headers: HashMap::new(),
    }
}

fn route(path: &String) -> (StatusCode, String) {
    let path = path.as_str();

    let status_code = match path {
        _path if is_valid_path(path) => StatusCode::Ok,
        _ => StatusCode::BadRequest,
    };
    let content = get_content(path, &status_code);

    (status_code, content)
}

fn get_content(path: &str, status_code: &StatusCode) -> String {
    match status_code {
        StatusCode::Ok => match path {
            _ if path.starts_with("/echo/") => path.to_string(),
            _ if path.starts_with("/sleep") => {
                thread::sleep(Duration::from_secs(5));
                String::from("woke from sleep!")
            }
            _ => get_html_content("static/hello_world.html"),
        },
        StatusCode::BadRequest => get_html_content("static/bad_request.html"),
    }
}

fn is_valid_path(path: &str) -> bool {
    match path {
        "/" => true,
        _ if path.starts_with("/echo/") => true,
        _ if path.starts_with("/sleep") => true,
        _ => false,
    }
}

fn get_html_content(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read file {file_path}")
}
