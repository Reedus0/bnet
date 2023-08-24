use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

#[derive(Debug)]
pub struct Client {
    pub ip: String,
    pub action: String,
    pub result: String
}

impl Client {
    pub fn get_ip_mut(&mut self) -> &mut String {
        &mut self.ip
    }
    pub fn get_action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    pub fn get_result_mut(&mut self) -> &mut String {
        &mut self.result
    }
    pub fn get_result(&self) -> &String {
        &self.result
    }
    pub fn get_action(&self) -> &String {
        &self.action
    }
    pub fn get_ip(&self) -> &String {
        &self.ip
    }
}

#[derive(Debug)]
pub struct Request {
    pub syl: char,
    pub num: char,
    pub msg: String,
}

impl Request {
    pub fn serialize(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn deserialize(vector: Vec<u8>) -> Request {
        Request {
            syl: vector[0] as char,
            num: vector[1] as char,
            msg: String::from_utf8(vector[3..].to_vec()).unwrap(),
        }
    }

    pub fn get_msg(&self) -> &String {
        &self.msg
    }
}

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

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.syl == other.syl && self.num == other.num
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} {}", self.syl, self.num, self.msg)
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ip, self.action, self.result)
    }
}

pub static TRY_ESTABLISH_CONNECTION: Request = Request {
    syl: 'C',
    num: '0',
    msg: String::new(),
};

pub static CONNECTION_ESTABLISHED: Request = Request {
    syl: 'C',
    num: '1',
    msg: String::new(),
};

pub static CONNECTION_CLOSED: Request = Request {
    syl: 'C',
    num: '2',
    msg: String::new(),
};

pub static REQUEST_NONE: Request = Request {
    syl: 'N',
    num: '0',
    msg: String::new(),
};

pub static REQUEST_ERROR: Request = Request {
    syl: 'E',
    num: '0',
    msg: String::new(),
};

pub static PING_REQUEST: Request = Request {
    syl: 'P',
    num: '0',
    msg: String::new(),
};

pub static PING_RESPONSE: Request = Request {
    syl: 'P',
    num: '1',
    msg: String::new(),
};

pub static ACTION_REQUEST: Request = Request {
    syl: 'A',
    num: '0',
    msg: String::new(),
};

pub static ACTION_FOR: Request = Request {
    syl: 'A',
    num: '1',
    msg: String::new(),
};

pub static ACTION_FOR_OK: Request = Request {
    syl: 'A',
    num: '4',
    msg: String::new(),
};

pub static ACTION_RESPONSE: Request = Request {
    syl: 'A',
    num: '2',
    msg: String::new(),
};

pub static ACTION_RESULT: Request = Request {
    syl: 'A',
    num: '5',
    msg: String::new(),
};

pub static GET_ACTION_RESULT: Request = Request {
    syl: 'A',
    num: '6',
    msg: String::new(),
};

pub static ACTION_NONE: Request = Request {
    syl: 'A',
    num: '3',
    msg: String::new(),
};

pub static LOGIN_REQUEST: Request = Request {
    syl: 'L',
    num: '0',
    msg: String::new(),
};

pub static LOGIN_RESPONSE: Request = Request {
    syl: 'L',
    num: '1',
    msg: String::new(),
};

pub static LOGIN_ERROR: Request = Request {
    syl: 'L',
    num: '2',
    msg: String::new(),
};

pub static USER_REQUEST: Request = Request {
    syl: 'U',
    num: '0',
    msg: String::new(),
};

pub static USER_RESPONSE: Request = Request {
    syl: 'U',
    num: '1',
    msg: String::new(),
};