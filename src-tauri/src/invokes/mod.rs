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
    code: u8, // 1:成功 0:失败
    msg: Option<String>,
    data: Option<T>,
}

impl<T> Response<T> {
    fn new(code: u8, msg: Option<String>, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    #[allow(dead_code)]
    pub fn set_msg(&mut self, msg: &str) {
        self.msg = Some(msg.to_owned());
    }

    #[allow(dead_code)]
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn success() -> Self {
        Self::new(1, Some("success".to_owned()), None)
    }

    pub fn success_with_data(data: T) -> Self {
        Self::new(1, Some("success".to_owned()), Some(data))
    }

    pub fn fail() -> Self {
        Self::new(0, Some("fail".to_owned()), None)
    }
}
