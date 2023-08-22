use std::fmt;
use std::str::from_utf8;

#[derive(Debug)]
pub struct Request<'a> {
    syl: char,
    num: char,
    msg: &'a str,
}

pub enum ClientState {
    Unconnected,
    Connected,
}

impl Request<'_> {
    pub fn serialize(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn deserialize(vector: &Vec<u8>) -> Request<'_> {
        Request {
            syl: vector[0] as char,
            num: vector[1] as char,
            msg: from_utf8(&vector[3..]).unwrap(),
        }
    }
}

impl fmt::Display for ClientState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ClientState::Unconnected => write!(f, "Unconnected"),
            ClientState::Connected => write!(f, "Connected"),
        }
    }
}

impl PartialEq for Request<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.syl == other.syl && self.num == other.num
    }
}

impl fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} {}", self.syl, self.num, self.msg)
    }
}

pub const CONNECTION_REQUEST: Request = Request {
    syl: 'C',
    num: '0',
    msg: "connection request",
};

pub const NOT_ESTABLISHED_CONNECTION: Request = Request {
    syl: 'C',
    num: '3',
    msg: "not established connection",
};

pub const TRY_ESTABLISH_CONNECTION: Request = Request {
    syl: 'C',
    num: '2',
    msg: "try establish connection",
};

pub const ESTABLISHED_CONNECTION: Request = Request {
    syl: 'C',
    num: '1',
    msg: "established connection",
};

pub const PING_REQUEST: Request = Request {
    syl: 'P',
    num: '0',
    msg: "ping request",
};

pub const PING_RESPONSE: Request = Request {
    syl: 'P',
    num: '1',
    msg: "ping response",
};

pub const LOGIN_REQUEST: Request = Request {
    syl: 'A',
    num: '0',
    msg: "login request",
};

pub const LOGIN_RESPONSE: Request = Request {
    syl: 'A',
    num: '1',
    msg: "login response",
};

pub const LOGIN_ERROR: Request = Request {
    syl: 'A',
    num: '2',
    msg: "login error",
};
