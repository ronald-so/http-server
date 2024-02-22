use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

struct RequestLine {
    method: String,
    path: String,
    version: String,
}

fn main() {
    let port = "4221";
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr).unwrap();

    println!("http server started!");
    println!("listening for connections on port {port}");

    listen(listener);
}

fn listen(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("request: {:#?}", http_request);

    let request_line = get_request_line(http_request);
    let response = match request_line {
        Some(request_line) => route(&request_line),
        None => return,
    };

    stream
        .write_all(format!("{response}\r\n\r\n").as_bytes())
        .unwrap_or_else(|e| {
            eprintln!("failed to write to buffer. error: {e}");
        });
    println!("response: {response}");
    stream.flush().unwrap();
}

fn get_request_line(http_request: Vec<String>) -> Option<RequestLine> {
    let start_line = &http_request[0];
    let parts = start_line.split_whitespace().collect::<Vec<&str>>();
    let request_line: RequestLine;

    if let [method, path, version] = &parts[0..3] {
        request_line = RequestLine {
            method: method.to_string(),
            path: path.to_string(),
            version: version.to_string(),
        };
        Some(request_line)
    } else {
        None
    }
}

fn route(request_line: &RequestLine) -> String {
    let status_line = match request_line.path.as_str() {
        s if s.starts_with("/echo/") => "HTTP/1.1 200 OK",
        s if s.starts_with("/sleep") => "HTTP/1.1 200 OK",
        "/" => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 NOT FOUND",
    };
    let content_path = match request_line.path.as_str() {
        "/" => "static/hello_world.html",
        s if s.starts_with("/echo/") => "",
        s if s.starts_with("/sleep") => "",
        _ => "static/not_found.html",
    };
    let content = match request_line.path.as_str() {
        s if s.starts_with("/echo/") => {
            let (_prefix, suffix) = request_line.path.split_at(6);
            String::from(suffix)
        }
        s if s.starts_with("/sleep") => {
            thread::sleep(Duration::from_secs(5));
            String::from("woke from sleep!")
        }
        _ => fs::read_to_string(content_path).expect("failed to read file"),
    };
    let content_length = content.len();
    let content_length_line = format!("Content-Length: {content_length}");

    format!("{status_line}\r\n{content_length_line}\r\n\r\n{content}")
}
