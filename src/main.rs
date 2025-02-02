// https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Session

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to \"127.0.0.1:7878\"");

    let pool = ThreadPool::build(4).expect("Failed to initialize a thread pool of 4");

    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().expect("Failed to iterate to the next request").expect("Failed to read request");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };
    
    let contents = fs::read_to_string(filename).expect("Failed to read \"{filename}\"");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).expect("Failed to write response");
}