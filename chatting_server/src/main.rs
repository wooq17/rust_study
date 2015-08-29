use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep_ms;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;

use bincode::rustc_serialize::{encode, decode};

extern crate bincode;
extern crate rustc_serialize;

pub const MESSAGE_TAG_ISUUE_ID: u64 = 0x0;
pub const MESSAGE_TAG_CHAT: u64 = 0x1;
pub const MESSAGE_TAG_CLOSE: u64 = 0x2;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Header {
    pub length: usize,
    pub message_tag: u64,
    pub sender_id: u64,
}

pub const HEADER_SIZE: usize = 24;

struct Client {
    pub id: u64,
    pub stream: TcpStream,
    pub recv_buf : [u8; 512],
    pub read_buf : [u8; 512],
    pub end_idx: usize,
    pub tx: Sender<Signal>,
}

enum StreamState {
    Message(Header, String),
    NoMessage,
    Broken,
}

impl Client {
    /// 챗 그룹에서 메시지 전송할 때 사용할 스트림 반환 
    pub fn get_write_stream(&self) -> Option<TcpStream> {
        match self.stream.try_clone() {
            Ok(stream) => { Some(stream) },
            _ => { None }
        }
    }

    pub fn set_client_id(&mut self, id: u64) {
        self.id = id;
    }

    /// 클라이언트에서 전송한 데이터를 가공해서 완성된 패킷 형태로 채널 그룹으로 메시징
    pub fn cycle(&mut self) -> bool {
        match self.read_message() {
            StreamState::Message(header, message) => {
                self.tx.send(Signal::NewMessage(header, message));
                true
            },
            StreamState::NoMessage => { true },
            StreamState::Broken => { 
                self.tx.send(Signal::Close(self.id));
                false 
            },
        }
    }

    pub fn read_message(&mut self) -> StreamState {
        match self.stream.read(&mut self.read_buf) {
            Ok(recv_size) => {
                println!("after read {}", recv_size);

                if recv_size > 0 {
                    for idx in 0..recv_size {
                        self.recv_buf[self.end_idx] = self.read_buf[idx];
                        self.end_idx += 1;
                    }
                    
                    let (body_end, header, message) = self.handle_client_message();
                    
                    if body_end == 1024 {
                        return StreamState::Broken;
                    } else if body_end > 0 {
                        for idx in 0..(self.end_idx-body_end) {
                            self.recv_buf[idx] = self.recv_buf[body_end+idx];
                        }
                        
                        self.end_idx -= body_end;
                    
                        return StreamState::Message(header.unwrap(), message.unwrap());
                    }
                    return StreamState::NoMessage
                }

                StreamState::Broken
            },
            Err(_) => {
                println!("stream error");
                StreamState::Broken
            },
        }
    }

    fn handle_client_message(&mut self) -> (usize, Option<Header>, Option<String>) {
        let mut header: Header = decode(&self.recv_buf[0..HEADER_SIZE]).unwrap();
        let body_end = (header.length + HEADER_SIZE) as usize;
        
        if self.end_idx < body_end { 
            return (0, None, None); 
        }
        
        match header.message_tag {
            MESSAGE_TAG_CHAT => {
                let message = String::from_utf8_lossy(&self.recv_buf[HEADER_SIZE..body_end]);
                // println!("{}", message);

                header.sender_id = self.id;
                
                (body_end, Some(header), Some(message.to_string()))
            },
            MESSAGE_TAG_CLOSE => {
                (1024, None, None)
            }
            _ => { (body_end, None, None) },
        }
    }
}

fn read_client_stream(mut client: Client) {
    loop {
        if client.cycle() { continue; }
        else { break; }
    }
}

enum Signal {
    NewClient(Client),
    NewMessage(Header, String),
    Close(u64),
}

struct ChatGroup {
    pub client_streams: HashMap<u64, TcpStream>,
    pub last_issued_client_id: u64,
    pub tx: Sender<Signal>,
    pub rx: Receiver<Signal>
}

impl ChatGroup {
    pub fn new() -> ChatGroup {
        let (_tx, _rx): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        ChatGroup{ client_streams: HashMap::new(), last_issued_client_id: 0, tx: _tx, rx: _rx }
    }

    pub fn get_transmitter(&self) -> Option<Sender<Signal>> {
        Some(self.tx.clone())
    }

    pub fn cycle(&mut self) {
        // broadcast messages
        println!("check the recv msg");
        loop {
            match self.rx.recv() {
                Ok(signal) => {
                    match signal {
                        Signal::NewClient(mut new_client) => {
                            self.last_issued_client_id += 1;

                            new_client.set_client_id(self.last_issued_client_id);

                            let mut issue_id_header = Header{ length:0, message_tag:MESSAGE_TAG_ISUUE_ID, sender_id: self.last_issued_client_id };

                            let mut client_stream = new_client.get_write_stream().unwrap();

                            let header_bytes = encode(&issue_id_header, bincode::SizeLimit::Infinite).unwrap();
                            let mut sent = client_stream.write(&header_bytes[..]).unwrap();
                            match client_stream.flush() {
                                Ok(_) => println!("[send] {:?} bytes", sent),
                                _ => {},
                            }

                            self.client_streams.insert(self.last_issued_client_id, client_stream);
                            thread::spawn(move|| {
                                read_client_stream(new_client); // add channel
                            });

                            println!("[DEBUG][CLIENT] added");
                        },
                        Signal::NewMessage(new_header, new_message) => {
                            println!("message : {}", new_message);

                            let message_bytes = new_message.into_bytes();
                            let header_bytes = encode(&new_header, bincode::SizeLimit::Infinite).unwrap();

                            for (id, each_client_stream) in &mut self.client_streams {
                                let mut sent = each_client_stream.write(&header_bytes[..]).unwrap();
                                sent += each_client_stream.write(&message_bytes[..]).unwrap();
                                
                                match each_client_stream.flush() {
                                    Ok(_) => println!("[send] {:?} bytes", sent),
                                    _ => {},
                                }
                            }
                        },
                        Signal::Close(id) => {
                            let mut close_header = Header{ length:0, message_tag:MESSAGE_TAG_CLOSE, sender_id: id };

                            let header_bytes = encode(&close_header, bincode::SizeLimit::Infinite).unwrap();
                            let mut sent = self.client_streams.get(&id).unwrap().write(&header_bytes[..]).unwrap();
                            match self.client_streams.get(&id).unwrap().flush() {
                                Ok(_) => println!("[send] {:?} bytes", sent),
                                _ => {},
                            }

                            self.client_streams.remove(&id);
                        }
                    }
                },
                _ => { break; }
            }
        }

        println!("cycle ended");
    }
}

fn handle_chat_group(mut chat_group: ChatGroup) {
    chat_group.cycle();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    
    println!("Start to listen, ready to accept");

    let sample_chat_group = ChatGroup::new();
    let _tx = sample_chat_group.get_transmitter().unwrap();

    thread::spawn(move|| {
        handle_chat_group(sample_chat_group);
    });

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(new_stream) => {
                println!("[DEBUG][STREAM] new stream");

                let mut new_client = Client{ id:0, stream: new_stream, recv_buf: [0;512], read_buf: [0;512], end_idx: 0, tx: _tx.clone() };

                _tx.send(Signal::NewClient(new_client));
            }
            Err(e) => { /* connection failed */ }
        }
    }
    
    // close the socket server
    drop(listener);
}