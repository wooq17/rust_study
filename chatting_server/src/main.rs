use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep_ms;

extern crate bincode;
extern crate rustc_serialize;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Header {
    pub length: usize,
    pub message_id: u64,
    pub group_id: u64,
}

fn client_health_check(mut stream: TcpStream) {
    println!("Start health check for the new connection");

    let mut count = 0;

    loop {
        let message = format!("Health check {}", count);
        let mut header = Header{ length:0, message_id:2, group_id:0 };
        header.length = message.len();
        let header_bytes = bincode::encode(&header, bincode::SizeLimit::Infinite).unwrap();

        stream.write(&header_bytes[..]);
        stream.write(&message.into_bytes()[..]);
        stream.flush();

        sleep_ms(5000);
        count += 1;
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("Start to handle the new connection");

    loop {
        let mut read_buf : [u8; 128] = [0;128];
        let recv_size = stream.read(&mut read_buf).unwrap();

        let mut header = Header{ length:0, message_id:1, group_id:0 };
        header.length = recv_size;
        let header_bytes = bincode::encode(&header, bincode::SizeLimit::Infinite).unwrap();

        stream.write(&header_bytes[..]);
        stream.write(&read_buf[0..recv_size]);
        stream.flush();
    }
}

#[allow(unstable)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    
    println!("Start to listen, ready to accept");
    
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut clone = stream.try_clone().unwrap();
                thread::spawn(move|| {
                    // connection succeeded
                    client_health_check(clone)
                });
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    
    // close the socket server
    drop(listener);
}