use bnet_core::utils;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            let ping_result = send_request(&stream, utils::PING_REQUEST, utils::PING_RESPONSE);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send_request(mut stream: &TcpStream, request: utils::Request, response: utils::Request) -> bool {
    let serialized_request = request.serialize();
    let msg: &[u8] = &&serialized_request;
    stream.write(msg).unwrap();

    let mut data = [0 as u8; 50];

    match stream.read(&mut data) {
        Ok(size) => {
            println!("{}", from_utf8(&data).unwrap());
            let data_vector = data.to_vec();
            let response_object = utils::Request::deserialize(&data_vector);
            response_object == response
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    }
}
