#![allow(unused, dead_code)]
use std::io::Write;
use std::str::FromStr;
use std::{net::TcpListener, io::Read};
use std::path::PathBuf;


struct HTTPRequest {
    method: String,
    path: PathBuf,
    version: String,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming(){
        let mut http_request;
        let mut stream = stream.unwrap();
        let mut buffer = vec![0u8; 8192];
        let read_amount = stream.read(&mut buffer).unwrap();
        let request = std::str::from_utf8(&buffer[..read_amount]).unwrap();
        if let Some(first_line) = request.lines().next(){
            let header:Vec<&str> = first_line.split(' ').collect();
            if header.len() != 3{
                continue;
            }
            http_request = HTTPRequest{ method: header[0].to_owned(), path: PathBuf::from_str(header[1]).unwrap(), version: header[2].to_owned() };
            let response = "HTTP/1.1 200 OK".to_owned();

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
