use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

extern crate web_server;
use web_server::ThreadPool;

static ADDR: &str = "127.0.0.1:7878";
static NUM_THREADS: usize = 8;

fn main() {
    let listener = TcpListener::bind(ADDR).expect(&format!("could not bind to addr {}", ADDR));
    //single_blocking_thread(listener);
    limited_thread_pool(listener);
}

pub fn limited_thread_pool(listener: TcpListener) {
    let pool = ThreadPool::new(NUM_THREADS);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}
// spawns one OS thread per connection, vulnerable to DDOS
pub fn naive_thread_pool(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

// this blocks all other requests on request a request to /sleep
pub fn single_blocking_thread(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let mut contents = String::new();

    let (status, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_name).expect("failed to open file");

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status, contents);

    stream
        .write(response.as_bytes())
        .expect("failed to write response");
    stream.flush().expect("failed to flush response");
}
