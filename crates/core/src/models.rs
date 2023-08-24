use std::fmt;

#[derive(Debug)]
pub struct Client {
    pub ip: String,
    pub action: String,
    pub result: String,
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

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ip, self.action, self.result)
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
