use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

// awesome webserver (in rust)

fn main() {
    // `bind` works like a `new` function would -- its just called bind because connecting to a port is known as "binding to a port"
    // it also returns a `Result<T, E>` because binding can fail, because this is a small project it can just be unwrapped, any errors stopping the program
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // `incoming` returns an iterator which contains `TcpStream`s. One stream is an open connection between the client and server
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_>  = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()) // take_while() translation: iterates through things if something is true
        .collect();

        let response = "HTTP/1.1 200 OK\r\n\r\n"; // status line
        stream.write_all(response.as_bytes()).unwrap();
        println!("Request {:#?}", http_request)
}
