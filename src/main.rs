#![allow(unused, dead_code)]
mod http_server;
mod websocket_server;
use std::thread::{self, Thread};
fn main() {

    let http_server = thread::spawn(http_server::HTTPServer::listen);
    let websocket_server = thread::spawn(websocket_server::WebsocketServer::listen);
    http_server.join().unwrap();
    websocket_server.join().unwrap();
}
