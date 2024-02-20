use std::io::Write;
use std::net::{TcpListener, TcpStream};

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
    let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
    stream.write_all(response).unwrap_or_else(|e| {
        println!("failed to write to buffer. error: {e}");
    });
}
