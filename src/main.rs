use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, thread, time::Duration
};

fn main() {
    let listener = 
    TcpListener::bind("127.0.0.1:7878").
    unwrap();
    for stream in 
    listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new
    (&mut stream);
    let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    
    let (status_line, contents) = if !http_request.is_empty() && http_request[0].starts_with("GET / ") {
        ("HTTP/1.1 200 OK", fs::read_to_string("src/templates/hello.html").unwrap_or_else(|_| String::from("File not found")))
    } else if !http_request.is_empty() && http_request[0].starts_with("GET /sleep "){
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", fs::read_to_string("src/templates/hello.html").unwrap_or_else(|_| String::from("File not found")))
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("src/templates/404.html").unwrap_or_else(|_| String::from("404 Not Found")))
    };
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    let _res = stream.write_all(
        response.as_bytes()
    );

}
