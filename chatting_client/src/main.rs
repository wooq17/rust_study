use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::thread::sleep_ms;
use std::sync::{Arc, Mutex};

extern crate bincode;
extern crate rustc_serialize;

pub const MESSAGE_TAG_HEALTH_CHECK: u64 = 0x0;
pub const MESSAGE_TAG_ISUUE_ID: u64 = 0x1;
pub const MESSAGE_TAG_CHAT: u64 = 0x2;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Header {
    pub length: usize,
    pub message_tag: u64,
    pub group_id: u64,
    pub sender_id: u64,
}

struct Client {
    pub id: u64,
    pub stream: TcpStream,
    pub recv_buf : [u8; 2048],
    pub read_buf : [u8; 128],
    pub end_idx: usize,
}

impl Client {
    /// 챗 그룹에서 메시지 전송할 때 사용할 스트림 반환 
    pub fn get_write_stream(&self) -> Option<TcpStream> {
        match self.stream.try_clone() {
            Ok(stream) => { Some(stream) },
            _ => { None }
        }
    }

    pub fn read_message(&mut self) {
        let recv_size = self.stream.read(&mut self.read_buf).unwrap();

        if recv_size > 0 {
            for idx in 0..recv_size {
                self.recv_buf[self.end_idx] = self.read_buf[idx];
                self.end_idx += 1;
            }
            
            let (body_end, sender_id, message) = self.handle_server_message();
            
            // shift
            if body_end > 0 {
                for idx in 0..(self.end_idx-body_end) {
                    self.recv_buf[idx] = self.recv_buf[body_end+idx];
                }
                
                self.end_idx -= body_end;
            
                println!("{} >>> {}", sender_id, message.unwrap());
            }
        }
    }

    fn handle_server_message(&mut self) -> (usize, u64, Option<String>) {
        let header: Header = bincode::decode(&self.recv_buf[0..32]).unwrap();
        let body_end = (header.length + 32) as usize;
        
        if self.end_idx < body_end { 
            return (0, 0, None); 
        }
        
        match header.message_tag {
            MESSAGE_TAG_CHAT => {
                let message = String::from_utf8_lossy(&self.recv_buf[32..body_end]);
                // println!("{}", message);
                
                (body_end, header.sender_id, Some(message.to_string()))
            },
            _ => { (body_end, 0, None) },
        }
    }
}

fn fetch_from_server(mut client: Client) {
    loop {
        client.read_message();
    }
}

fn main() {
    let mut new_stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    
    println!("connected");
    
    let mut client = Client{ id:0, stream: new_stream, recv_buf: [0;2048], read_buf: [0;128], end_idx: 0 };
    let mut write_stream = client.get_write_stream().unwrap();
    
    let read_thread = thread::spawn(move|| { 
        fetch_from_server(client);
    });
    
    loop {
        let mut user_input: String = String::new();
	    io::stdin().read_line(&mut user_input);
        
        println!("[DEBUG][INPUT] {}", user_input);
        
        // make header
        let mut header = Header{ length:0, message_tag:MESSAGE_TAG_CHAT, group_id:0, sender_id: 0 };
        header.length = user_input.len();
        
        let message_bytes = user_input.into_bytes();
        let header_bytes = bincode::encode(&header, bincode::SizeLimit::Infinite).unwrap();

        let mut sent = write_stream.write(&header_bytes[..]).unwrap();
        sent += write_stream.write(&message_bytes[..]).unwrap();
        write_stream.flush();
    }
    
    let result = read_thread.join();
   
    println!("Close the connection");
}