use bnet_core::utils;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            let server_ok =
                utils::send_request_check(&stream, &utils::PING_REQUEST, &utils::PING_RESPONSE);

            if server_ok {
                println!("Enter password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                let LOGIN_REQUEST: utils::Request = utils::Request {
                    syl: 'L',
                    num: '0',
                    msg: password.trim().to_string(),
                };
                println!("{}", LOGIN_REQUEST);
                let logined = utils::send_request_check(&stream, &LOGIN_REQUEST, &utils::LOGIN_RESPONSE);
                if logined {
                    println!("Logined");
                    println!("Enter IP: ");
                    let mut ip: String = String::new();
                    std::io::stdin().read_line(&mut ip).unwrap();
                    println!("Enter action: ");
                    let mut action: String = String::new();
                    std::io::stdin().read_line(&mut action).unwrap();
                    let ACTION_FOR: utils::Request = utils::Request {
                        syl: 'A',
                        num: '1',
                        msg:  format!("{} {}", ip.trim(), action.trim()),
                    };
                    utils::send_request(&stream, &ACTION_FOR);
                    std::thread::sleep(std::time::Duration::from_millis(5000));
                } else {
                    println!("Didn't login");
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
