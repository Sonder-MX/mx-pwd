pub mod cipher_invoke;
mod test;

use serde::Serialize;
use serde_json::Value;

// 响应数据
#[derive(Serialize)]
pub struct ResponseData {
    pub code: i32,
    pub msg: String,
    pub data: Value,
}

impl ResponseData {
    pub fn new(code: i32, msg: String, data: Value) -> Self {
        Self { code, msg, data }
    }
}
