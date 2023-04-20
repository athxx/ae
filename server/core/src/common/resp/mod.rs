mod code;
mod error;
mod msg;

struct Resp {
    code: i8,
    msg: String,
    data: String,
}
