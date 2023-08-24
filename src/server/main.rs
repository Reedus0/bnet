use bnet_core::{config, models, protocol, utils};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

static mut USERS: Vec<models::Client> = Vec::new();

fn main() {
    let listener = TcpListener::bind(config::IP).unwrap();

    thread::scope(|scope: &thread::Scope<'_, '_>| {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    scope.spawn(|| handle_client(stream));
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    });

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let new_user = models::Client {
        ip: stream.peer_addr().unwrap().to_string(),
        action: String::new(),
        result: String::new(),
    };
    unsafe { &mut USERS.push(new_user) };
    let mut data = [0 as u8; 1000];

    while match stream.read(&mut data) {
        Ok(size) => {
            if data[0] == 0 {
                return;
            };
            println!(
                "{} {}",
                from_utf8(&data).unwrap(),
                stream.peer_addr().unwrap()
            );
            let data_vector = data[0..size].to_vec();
            let request = models::Request::deserialize(data_vector);

            if request == protocol::PING_REQUEST {
                utils::send_response(&stream, &protocol::PING_RESPONSE);
                data = [0 as u8; 1000];
            }

            if request == protocol::TRY_ESTABLISH_CONNECTION {
                utils::send_response(&stream, &protocol::CONNECTION_ESTABLISHED);
                data = [0 as u8; 1000];
            }

            if request == protocol::CONNECTION_CLOSED {
                utils::send_response(&stream, &protocol::CONNECTION_CLOSED);
                let user = get_user(stream.peer_addr().unwrap().to_string());
                unsafe { &mut USERS.remove(USERS.iter().position(|x| *x.ip == user.ip).unwrap()) };
                data = [0 as u8; 1000];
            }

            if request == protocol::LOGIN_REQUEST {
                if request.get_msg() == unsafe { &config::PASSWORD.to_string() } {
                    utils::send_response(&stream, &protocol::LOGIN_RESPONSE);
                    data = [0 as u8; 1000];
                } else {
                    utils::send_response(&stream, &protocol::LOGIN_ERROR);
                    data = [0 as u8; 1000];
                }
            }

            if request == protocol::USER_REQUEST {
                let USER_RESPONSE: models::Request = models::Request {
                    syl: 'U',
                    num: '1',
                    msg: format!("{:?}", unsafe { &USERS }),
                };

                utils::send_response(&stream, &USER_RESPONSE);
                data = [0 as u8; 1000];
            }

            if request == protocol::ACTION_FOR {
                let msg: Vec<&str> = request.get_msg().split(" ").collect();
                set_action(msg[1..].join(" ").to_string(), msg[0].to_string());
                set_result("".to_string(), msg[0].to_string());
                utils::send_response(&stream, &protocol::ACTION_FOR_OK);
                data = [0 as u8; 1000];
            }

            if request == protocol::ACTION_RESULT {
                set_action("".to_string(), stream.peer_addr().unwrap().to_string());
                set_result(
                    request.get_msg().to_string(),
                    stream.peer_addr().unwrap().to_string(),
                );

                utils::send_response(&stream, &protocol::ACTION_NONE);
                data = [0 as u8; 1000];
            }

            if request == protocol::GET_ACTION_RESULT {
                let msg: Vec<&str> = request.get_msg().split(" ").collect();
                let current_user = get_user(msg[0].to_string());
                let ACTION_RESULT: models::Request = models::Request {
                    syl: 'A',
                    num: '5',
                    msg: current_user.get_result().to_string(),
                };
                utils::send_response(&stream, &ACTION_RESULT);
                data = [0 as u8; 1000];
            }

            if request == protocol::ACTION_REQUEST {
                let current_user = get_user(stream.peer_addr().unwrap().to_string());
                if current_user.action != "" {
                    let ACTION_RESPONSE: models::Request = models::Request {
                        syl: 'A',
                        num: '2',
                        msg: current_user.get_action().to_string(),
                    };
                    utils::send_response(&stream, &ACTION_RESPONSE);
                    data = [0 as u8; 1000];
                } else {
                    utils::send_response(&stream, &protocol::ACTION_NONE);
                    data = [0 as u8; 1000];
                }
            }

            true
        }
        Err(e) => {
            utils::send_response(&stream, &protocol::REQUEST_ERROR);
            data = [0 as u8; 1000];
            false
        }
    } {}
}

fn set_action(action: String, ip: String) {
    for client in unsafe { &mut USERS } {
        if client.ip == ip {
            *client.get_action_mut() = action.clone();
        }
    }
}

fn set_result(result: String, ip: String) {
    for client in unsafe { &mut USERS } {
        if client.ip == ip {
            *client.get_result_mut() = result.clone();
        }
    }
}

fn get_user(ip: String) -> &'static mut models::Client {
    let mut index = 0;
    unsafe {
        for i in 1..USERS.len() {
            if USERS[i].ip == ip {
                index = i;
            }
        }

        USERS.get_mut(index).unwrap()
    }
}
