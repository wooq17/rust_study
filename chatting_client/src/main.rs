use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
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

fn fetch_from_sever_message(mut stream: TcpStream) {
    let mut recv_buf : [u8; 2048] = [0;2048];
    let mut end_idx = 0;
    
    loop {
        println!("loop");
        let mut read_buf : [u8; 128] = [0;128];
        let recv_size = stream.read(&mut read_buf).unwrap();
        
        if recv_size > 0 {
            println!("[DEBUG][recv={}]", recv_size);
            println!("[DEBUG] {}", String::from_utf8_lossy(&read_buf[..]));
            
            for idx in 0..recv_size {
                recv_buf[end_idx] = read_buf[idx];
                end_idx += 1;
            }
            
            let body_end = handle_sever_message(&recv_buf, end_idx);
            
            // shift
            if body_end > 0 {
                for idx in 0..(end_idx-body_end) {
                    recv_buf[idx] = recv_buf[body_end+idx];
                }
                
                end_idx -= body_end;
            }
        }
    }
}

fn handle_sever_message(recv: &[u8], end_idx: usize) -> usize {
    println!("[DEBUG][end_idx={}]", end_idx);

    let header: Header = bincode::decode(&recv[0..24]).unwrap();
    println!("[DEBUG] length={}, message_id={}", header.length, header.message_id);
    
    match header.message_id {
        1 => {
            println!("[DEBUG] MESSAGE");
            
            let body_end = (header.length + 24) as usize;
            
            if end_idx < body_end { 
                println!("end_idx={} body_end={}", end_idx, body_end);
                return 0; 
            }
            
            let message = String::from_utf8_lossy(&recv[24..body_end]);
            println!("{}", message);
            
            body_end
        },
        2 => {
            println!("[DEBUG] HEALTH CHECK");
            
            let body_end = (header.length + 24) as usize;
            
            if end_idx < body_end { 
                println!("end_idx={} body_end={}", end_idx, body_end);
                return 0; 
            }
            
            let message = String::from_utf8_lossy(&recv[24..body_end]);
            println!("{}", message);
            
            body_end
        },
        _ => { 0 },
    }
}

fn send_message(header: Header, message: &[u8]) {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    
    let header_bytes = bincode::encode(&header, bincode::SizeLimit::Infinite).unwrap();
    
    let mut send = stream.write(&header_bytes[..]).unwrap();
    send += stream.write(message).unwrap();
    
    match stream.flush() {
        Ok(_) => println!("[send] {:?} bytes", send),
        _ => {},
    }
}

#[allow(unstable)]
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    let mut clone = stream.try_clone().unwrap();
    let read_thread = thread::spawn(move|| { fetch_from_sever_message(clone) });
    
    loop {
        let mut user_input: String = String::new();
	    io::stdin().read_line(&mut user_input);
        
        stream.write(&user_input.into_bytes()[..]);
    }
    
    let result = read_thread.join();
   
    println!("Close the connection");
}