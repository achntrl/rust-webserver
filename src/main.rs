#[macro_use]
extern crate log;
extern crate env_logger;

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut message = [0; 1024*8];
    stream.read(&mut message)?;
    stream.write(b"HTTP/1.1 200 OK\r\n\r\n<h1>Hello World from Rust</h1>\r\n")?;
     Ok(())
}

fn main() {
    env_logger::init().unwrap();

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { thread::spawn(|| {
                let mut stream = stream;
                match handle_client(&mut stream) {
                    Ok(_) => { info!("Connection received form {}", stream.peer_addr().unwrap()); }
                    Err(_) => { error!("An error occured"); }
                }
            });}
            Err(_) => { println!("Error receiving !"); }
        }
    }
    println!("Hello, world!");
}
