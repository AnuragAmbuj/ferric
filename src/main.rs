mod command_handler;
mod resp_handler;
mod membership;
mod cache_ops;
mod event_loop_ops;
mod string_ops;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread};


use crate::command_handler::get_port;
use crate::resp_handler::handle_query_requests;

fn main() {
    let port = get_port();
    println!("Starting a new redis server on port {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    handle_new_connection(listener);
}

pub fn handle_new_connection(listener: TcpListener) {
    for tcp_stream_connection in listener.incoming() {
        match tcp_stream_connection {
            Ok(conn_stream) => {
                thread::spawn(|| {
                    handle_request(conn_stream)
                });
            }
            Err(e) => {
                panic!("Fatal error, cannot read TCP stream {}", e)
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut next = true;
    while next {
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();
        handle_query_requests(buffer, &mut stream);
        next = len > 0;
    }
}