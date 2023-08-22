use bnet_core::utils;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            loop {
                send_request(&stream, &utils::PING_REQUEST, &utils::PING_RESPONSE);
                std::thread::sleep(std::time::Duration::from_millis(5000))
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send_request(
    mut stream: &TcpStream,
    request: &utils::Request,
    response: &utils::Request,
) -> bool {
    let request_string = request.to_string();
    let msg: &[u8] = request_string.as_bytes();
    stream.write(msg).unwrap();

    let mut data = [0 as u8; 50];

    let result = match stream.read(&mut data) {
        Ok(size) => {
            println!("{}", from_utf8(&data).unwrap());
            true
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    };
    result
}
