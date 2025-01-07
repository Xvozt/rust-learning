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

    // let _http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("hello.html").unwrap();
        let length = content.len();
        let headers = format!(
            "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n",
        );

        stream.write_all(headers.as_bytes()).unwrap();
        stream.write_all(content.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("not_found.html").unwrap();
        let length = content.len();
        let headers = format!(
            "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n",
        );
        stream.write_all(headers.as_bytes()).unwrap();
        stream.write_all(content.as_bytes()).unwrap();
    }

    //Separate headers and content

    stream.flush().unwrap();
}
