use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "not_found.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let headers =
        format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n",);

    stream.write_all(headers.as_bytes()).unwrap();
    stream.write_all(content.as_bytes()).unwrap();
    stream.flush().unwrap();
}
