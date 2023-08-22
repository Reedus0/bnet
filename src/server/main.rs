use bnet_core::utils;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];

    while match stream.read(&mut data) {
        Ok(size) => {
            println!("{}", from_utf8(&data).unwrap());
            let data_vector = data.to_vec();
            let request = utils::Request::deserialize(&data_vector);

            if request == utils::PING_REQUEST {
                send_response(&stream, &utils::PING_RESPONSE);
            } 
            true
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn send_response(mut stream: &TcpStream, response: &utils::Request) {
    let serialized_response = response.serialize();
    let msg: &[u8] = &&serialized_response;
    stream.write(msg).unwrap();
}
