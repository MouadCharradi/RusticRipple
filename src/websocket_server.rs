use std::thread;

use websocket::{sync::Server, OwnedMessage, Message};

pub struct WebsocketServer;

impl WebsocketServer {
    pub fn listen() {
        
        let server = Server::bind("127.0.0.1:8081").unwrap();

        for connection in server.filter_map(Result::ok) {
            // Spawn a new thread for each connection.
            thread::spawn(move || {
                let mut client = connection.accept().unwrap();
                let message = client.recv_message();
                if let Ok(msg) = message {
                    if let OwnedMessage::Text(text) = msg {
                        println!("{}", text)
                    }
                }
                let message = Message::text("Hello, client!");
                let _ = client.send_message(&message);
            });
        }
    }
}
