#![allow(unused, dead_code)]
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{io::Read, net::TcpListener};

struct HTTPRequest<'a> {
    method: &'a str,
    path: PathBuf,
    version: &'a str,
}

impl<'a> From<Vec<&'a str>> for HTTPRequest<'a> {
    fn from(header: Vec<&'a str>) -> Self {
        Self {
            method: header[0],
            path: PathBuf::from_str(header[1]).unwrap(),
            version: header[2],
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = vec![0u8; 8192];
        let read_amount = stream.read(&mut buffer).unwrap();
        let request = std::str::from_utf8(&buffer[..read_amount]).unwrap();
        if let Some(first_line) = request.lines().next() {
            let header: Vec<&str> = first_line.split(' ').collect();
            if header.len() != 3 {
                continue;
            }
            let http_request = HTTPRequest::from(header);
        }
        let mut response = "HTTP/1.1 200 OK\r\n\r\n".to_owned();
        response.push_str(include_str!("../public/index.html"));
        stream.write_all(response.as_bytes()).unwrap();
    }
}
