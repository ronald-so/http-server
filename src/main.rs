mod http;
mod router;

use http_server::thread_pool::ThreadPool;

use crate::http::parse_request;
use crate::router::handle_request;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
// use std::thread;

fn main() {
    let port = "4221";
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr).unwrap();
    let thread_pool = ThreadPool::new(4);

    println!("http server started!");
    println!("listening for connections on port {port}");

    listen(listener, thread_pool);
}

fn listen(listener: TcpListener, thread_pool: ThreadPool) {
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => thread_pool.execute(|| {
                handle_connection(_stream);
            }),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let raw_request: Vec<String> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = parse_request(&raw_request);
    let response = match request {
        Ok(res) => handle_request(&res),
        Err(_) => {
            eprintln!("Failed to parse request");
            return ();
        }
    };

    stream.write_all(&response.as_bytes()).unwrap_or_else(|e| {
        eprintln!("Failed to write to buffer. Error: {e}");
    });
    stream.flush().unwrap();
}
