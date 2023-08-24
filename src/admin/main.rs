use bnet_core::{
    models, protocol,
    utils::{self}, config,
};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect(config::IP) {
        Ok(stream) => {
            println!("Successfully connected to server in port 7878");

            let established_connection = utils::send_request_check(
                &stream,
                &protocol::TRY_ESTABLISH_CONNECTION,
                &protocol::CONNECTION_ESTABLISHED,
            );

            if established_connection {
                println!("Enter password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                let LOGIN_REQUEST: models::Request = models::Request {
                    syl: 'L',
                    num: '0',
                    msg: password.trim().to_string(),
                };
                println!("{}", LOGIN_REQUEST);
                let logined =
                    utils::send_request_check(&stream, &LOGIN_REQUEST, &protocol::LOGIN_RESPONSE);
                if logined {
                    println!("Logined");
                    utils::send_request(&stream, &protocol::USER_REQUEST);
                    loop {
                        println!("Enter IP: ");
                        let mut ip: String = String::new();
                        std::io::stdin().read_line(&mut ip).unwrap();
                        println!("Enter action: ");
                        let mut action: String = String::new();
                        std::io::stdin().read_line(&mut action).unwrap();
                        let ACTION_FOR: models::Request = models::Request {
                            syl: 'A',
                            num: '1',
                            msg: format!("{} {}", ip.trim(), action.trim()),
                        };
                        utils::send_request(&stream, &ACTION_FOR);
                        std::thread::sleep(std::time::Duration::from_millis(10000));
                        let GET_ACTION_RESULT: models::Request = models::Request {
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
                &protocol::CONNECTION_CLOSED,
                &protocol::CONNECTION_CLOSED,
            );
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
