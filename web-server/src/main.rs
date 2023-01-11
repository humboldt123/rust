use web_server::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// awesome webserver (in rust)

fn main() {
    // `bind` works like a `new` function would -- its just called bind because connecting to a port is known as "binding to a port"
    // it also returns a `Result<T, E>` because binding can fail, because this is a small project it can just be unwrapped, any errors stopping the program
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(4);

    // `incoming` returns an iterator which contains `TcpStream`s. One stream is an open connection between the client and server
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{contents}", contents.len());
        
    stream.write_all(response.as_bytes()).unwrap();

}
