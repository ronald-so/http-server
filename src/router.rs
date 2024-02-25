use crate::http::{HttpRequest, HttpResponse, StatusCode};

// pub fn handle_request(request: &HttpRequest) -> Result<HttpResponse, _> {
pub fn handle_request(request: &HttpRequest) {
    println!("handling request");
}

fn route(path: &String) -> StatusCode {
    let path = path.as_str();
    match path {
        _ => StatusCode::BadRequest,
    }
}

fn get_content(path: &String, status_code: &StatusCode) {
    match status_code {
        StatusCode::Ok => println!("200"),
        StatusCode::BadRequest => println!("400"),
    }
}
