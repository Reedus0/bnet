use bnet_core::utils::{self, Client, ClientState};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

static mut users: Vec<utils::Client> = Vec::new();

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    thread::scope(|scope: &thread::Scope<'_, '_>| {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    scope.spawn(|| unsafe { handle_client(stream) });
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
    let new_user = utils::Client {
        ip: stream.peer_addr().unwrap().to_string(),
        state: utils::ClientState::Unconnected,
        action: String::new(),
        result: String::new(),
    };
    unsafe { &mut users.push(new_user) };
    let mut data = [0 as u8; 1000];

    while match stream.read(&mut data) {
        Ok(size) => {
            if (data[0] == 0) {
                return;
            };
            println!(
                "{} {}",
                from_utf8(&data).unwrap(),
                stream.peer_addr().unwrap()
            );
            let data_vector = data[0..size].to_vec();
            let request = utils::Request::deserialize(data_vector);

            if request == utils::PING_REQUEST {
                utils::send_response(&stream, &utils::PING_RESPONSE);
                data = [0 as u8; 1000];
            }

            if request == utils::TRY_ESTABLISH_CONNECTION {
                utils::send_response(&stream, &utils::CONNECTION_ESTABLISHED);
                data = [0 as u8; 1000];
            }

            if request == utils::CONNECTION_CLOSED {
                utils::send_response(&stream, &utils::CONNECTION_CLOSED);
                let user = get_user(stream.peer_addr().unwrap().to_string());
                unsafe { &mut users.remove(users.iter().position(|x| *x.ip == user.ip).unwrap()) };
                data = [0 as u8; 1000];
            }

            if request == utils::LOGIN_REQUEST {
                if request.get_msg() == &String::from("12345678") {
                    utils::send_response(&stream, &utils::LOGIN_RESPONSE);
                    data = [0 as u8; 1000];
                    set_state(
                        utils::ClientState::Loged,
                        stream.peer_addr().unwrap().to_string(),
                    )
                } else {
                    utils::send_response(&stream, &utils::LOGIN_ERROR);
                    data = [0 as u8; 1000];
                }
            }

            if request == utils::USER_REQUEST {
                let USER_RESPONSE: utils::Request = utils::Request {
                    syl: 'U',
                    num: '1',
                    msg: format!("{:?}", unsafe { &users }),
                };

                &utils::send_response(&stream, &USER_RESPONSE);
                data = [0 as u8; 1000];
            }

            if request == utils::ACTION_FOR {
                let msg: Vec<&str> = request.get_msg().split(" ").collect();
                set_action(msg[1..].join(" ").to_string(), msg[0].to_string());
                set_state(utils::ClientState::Act, msg[0].to_string());
                set_result("".to_string(), msg[0].to_string());
                utils::send_response(&stream, &utils::ACTION_FOR_OK);
                data = [0 as u8; 1000];
            }

            if request == utils::ACTION_RESULT {
                set_action("".to_string(), stream.peer_addr().unwrap().to_string());
                set_state(
                    utils::ClientState::Unconnected,
                    stream.peer_addr().unwrap().to_string(),
                );
                set_result(
                    request.get_msg().to_string(),
                    stream.peer_addr().unwrap().to_string(),
                );

                utils::send_response(&stream, &utils::ACTION_NONE);
                data = [0 as u8; 1000];
            }

            if request == utils::GET_ACTION_RESULT {
                let msg: Vec<&str> = request.get_msg().split(" ").collect();
                let current_user = get_user(msg[0].to_string());
                let ACTION_RESULT: utils::Request = utils::Request {
                    syl: 'A',
                    num: '5',
                    msg: current_user.get_result().to_string(),
                };
                utils::send_response(&stream, &ACTION_RESULT);
                data = [0 as u8; 1000];
            }

            if request == utils::ACTION_REQUEST {
                let current_user = get_user(stream.peer_addr().unwrap().to_string());
                if current_user.state == utils::ClientState::Act {
                    let ACTION_RESPONSE: utils::Request = utils::Request {
                        syl: 'A',
                        num: '2',
                        msg: current_user.get_action().to_string(),
                    };
                    utils::send_response(&stream, &ACTION_RESPONSE);
                    data = [0 as u8; 1000];
                } else {
                    utils::send_response(&stream, &utils::ACTION_NONE);
                    data = [0 as u8; 1000];
                }
            }

            true
        }
        Err(e) => {
            utils::send_response(&stream, &utils::REQUEST_ERROR);
            data = [0 as u8; 1000];
            false
        }
    } {}
}

fn set_state(state: ClientState, ip: String) {
    for client in unsafe { &mut users } {
        if client.ip == ip {
            *client.get_state_mut() = state;
        }
    }
}

fn set_action(action: String, ip: String) {
    for client in unsafe { &mut users } {
        if client.ip == ip {
            *client.get_action_mut() = action.clone();
        }
    }
}

fn set_result(result: String, ip: String) {
    for client in unsafe { &mut users } {
        if client.ip == ip {
            *client.get_result_mut() = result.clone();
        }
    }
}

fn get_user(ip: String) -> &'static mut utils::Client {
    let mut index = 0;
    unsafe {
        for i in 1..users.len() {
            if (users[i].ip == ip) {
                index = i;
            }
        }

        users.get_mut(index).unwrap()
    }
}
