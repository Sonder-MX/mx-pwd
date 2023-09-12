pub mod cipher_invoke;
mod test;

use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    code: u16, // 200:success 400: fail
    msg: Option<String>,
    data: Option<T>,
}

impl<T> Response<T> {
    fn new(code: u16, msg: Option<String>, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn success(msg: Option<String>) -> Self {
        Self::new(200, msg, None)
    }

    pub fn success_with_data(data: T) -> Self {
        Self::new(200, None, Some(data))
    }

    pub fn error(msg: &str) -> Self {
        Self::new(400, Some(msg.to_string()), None)
    }
}
