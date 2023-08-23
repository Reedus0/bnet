use bnet_core::utils::{self};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

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
                println!("Enter password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                let LOGIN_REQUEST: utils::Request = utils::Request {
                    syl: 'L',
                    num: '0',
                    msg: password.trim().to_string(),
                };
                println!("{}", LOGIN_REQUEST);
                let logined =
                    utils::send_request_check(&stream, &LOGIN_REQUEST, &utils::LOGIN_RESPONSE);
                if logined {
                    println!("Logined");
                    utils::send_request(&stream, &utils::USER_REQUEST);
                    loop {
                        println!("Enter IP: ");
                        let mut ip: String = String::new();
                        std::io::stdin().read_line(&mut ip).unwrap();
                        println!("Enter action: ");
                        let mut action: String = String::new();
                        std::io::stdin().read_line(&mut action).unwrap();
                        let ACTION_FOR: utils::Request = utils::Request {
                            syl: 'A',
                            num: '1',
                            msg: format!("{} {}", ip.trim(), action.trim()),
                        };
                        utils::send_request(&stream, &ACTION_FOR);
                        std::thread::sleep(std::time::Duration::from_millis(10000));
                        let GET_ACTION_RESULT: utils::Request = utils::Request {
                            syl: 'A',
                            num: '6',
                            msg: ip.trim().to_string(),
                        };
                        utils::send_request(&stream, &GET_ACTION_RESULT);
                    }
                } else {
                    println!("Didn't login");
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
