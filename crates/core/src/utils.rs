use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use crate::models::Request;

pub fn send_response<'a>(mut stream: &TcpStream, response: &Request) -> bool{
    let serialized_response = response.serialize();
    let msg: &[u8] = &&serialized_response;
    match stream.write(msg) {
        Ok(_) => true,
        Err(e) => {
            println!("Failed to send data: {}", e);
            false
        }
    }
}

pub fn send_request<'a>(mut stream: &TcpStream, request: &Request) -> Request {
    let serialized_request = request.serialize();
    let msg: &[u8] = &&serialized_request;
    match stream.write(msg) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to send data: {}", e)
        }
    }

    let mut data = [0 as u8; 1000];

    match stream.read(&mut data) {
        Ok(size) => {
            println!(
                "{} {}",
                from_utf8(&data).unwrap(),
                stream.peer_addr().unwrap()
            );
            let data_vector = data[0..size].to_vec();
            let response_object = Request::deserialize(data_vector);
            response_object
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            Request {
                syl: 'E',
                num: '0',
                msg: "error".to_string(),
            }
        }
    }
}

pub fn send_request_check(mut stream: &TcpStream, request: &Request, response: &Request) -> bool {
    let serialized_request = request.serialize();
    let msg: &[u8] = &&serialized_request;
    match stream.write(msg) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to send data: {}", e)
        }
    }

    let mut data = [0 as u8; 1000];

    match stream.read(&mut data) {
        Ok(size) => {
            println!(
                "{} {}",
                from_utf8(&data).unwrap(),
                stream.peer_addr().unwrap()
            );
            let data_vector = data[0..size].to_vec();
            let response_object = Request::deserialize(data_vector);
            &response_object == response
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    }
}