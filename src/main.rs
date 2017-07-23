#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut request = [0; 1024*2];
    stream.read(&mut request)?;
    let request_str = String::from_utf8(request.to_vec()).unwrap();
    let (method, path) = read_request(&request_str);
    println!("{:?} {} ", method, path);

    match method {
        RequestMethod::GET => { get(stream, &path)?; }
        RequestMethod::POST => { println!("POST: Not implemented yet");}
    }

     Ok(())
}

fn get(stream: &mut TcpStream, path: &str) -> io::Result<()> {
    let mut full_path: String = "/Users/alexandre/.dotfiles/Frontpage".to_string();
    full_path = if path == "/" { full_path + "/index.html" } else { full_path + path };
    let file = File::open(full_path)?;
    let mut buf_reader = BufReader::new(file);

    stream.write(b"HTTP/1.1 200 OK\r\n")?;
    if path.ends_with(".svg") {
        stream.write(b"Content-type:image/svg+xml;charset=UTF-8\r\n")?;
    }
    if path.ends_with(".png") {
        stream.write(b"Content-type:image/png;charset=UTF-8\r\n")?;
    }
    if path.ends_with(".js") {
        stream.write(b"Content-type:application/javascript;charset=UTF-8\r\n")?;
    }
    if path.ends_with(".ico") {
        stream.write(b"Content-type:image/ico;charset=UTF-8\r\n")?;
    }

    stream.write(b"\r\n")?;

    let mut served_page = Vec::new();
    buf_reader.read_to_end(&mut served_page)?;
    stream.write(&served_page)?;

    Ok(())
}

#[derive(Debug)]
enum RequestMethod {
    GET,
    POST,
}
fn read_request(request_str: &String) -> (RequestMethod, &str) {
    let request: Vec<&str> = request_str.split("\n").collect();
    let request_args: Vec<&str> = request[0].split(" ").collect();
    let request_method = match request_args[0] {
        "GET" => { RequestMethod::GET },
        "POST" => { RequestMethod::POST },
        _ => {panic!("Error: couldn't parse request type");}
    };
    let request_path = request_args[1];
    (request_method, &request_path)

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
                    Err(e) => { error!("An error occured: {}", e); }
                }
            });}
            Err(_) => { println!("Error receiving !"); }
        }
    }
    println!("Hello, world!");
}
