use crate::models::Request;

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
