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

#[derive(Serialize)]
pub struct Response<T> {
    code: u8, // 0:成功 1:失败
    msg: Option<String>,
    data: Option<T>,
}

impl<T> Response<T> {
    fn new(code: u8, msg: Option<String>, data: Option<T>) -> Self {
        Response { code, msg, data }
    }

    #[allow(dead_code)]
    pub fn set_code(&mut self, code: u8) {
        self.code = code;
    }

    #[allow(dead_code)]
    pub fn set_msg(&mut self, msg: String) {
        self.msg = Some(msg);
    }

    pub fn success() -> Self {
        Self::new(0, None, None)
    }

    pub fn success_with_data(msg: Option<String>, data: Option<T>) -> Self {
        Self::new(0, msg, data)
    }

    pub fn error(msg: String) -> Self {
        Self::new(1, Some(msg), None)
    }
}
