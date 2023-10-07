#![allow(unused, dead_code)]
use std::io::Write;
use std::path::{PathBuf, Component, Path};
use std::str::FromStr;
use std::{io::Read, net::TcpListener};
use std::fs;
use std::env::current_dir;
use extend::ext;

use websocket::codec::http;

#[ext]
impl Path {
    fn resolve_inside(&self) -> PathBuf {
        // let new = parent.as_ref().to_path_buf();
        let mut new = PathBuf::new();
        for component in self.components() {
            match component {
                Component::ParentDir => {
                    new.pop();
                }
                Component::Normal(normal) => new.push(normal),
                _ => {}
            }
        }
        new
    }
}

struct HTTPRequest<'a> {
    method: &'a str,
    path: PathBuf,
    version: &'a str,
}

impl<'a> From<Vec<&'a str>> for HTTPRequest<'a> {
    fn from(header: Vec<&'a str>) -> Self {
        Self {
            method: header[0],
            path: PathBuf::from(header[1]).resolve_inside(),
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
            println!("{}",http_request.path.to_str().unwrap());
            let root = current_dir().expect("Hardcoded path should never fail").join("public");

            let file = fs::read_to_string(root.join(http_request.path));
            match file{
                Ok(file) => {
                    let mut response = "HTTP/1.1 200 OK\r\n\r\n".to_owned();
                    response.push_str(file.as_str());
                    stream.write_all(response.as_bytes()).unwrap();
                },
                Err(err) => {
                    let mut response = "HTTP/1.1 404 OK\r\n\r\n".to_owned();
                    response.push_str(&err.to_string()[..]);
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        }
    }
}
