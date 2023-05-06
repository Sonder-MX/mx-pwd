pub mod create_tb;
pub mod delete;
pub mod insert;
pub mod update;

use rusqlite::Connection;
use std::ops::Add;
use std::sync::Mutex;

pub struct DBState {
    pub conn: Mutex<Connection>, // Mutex是一个互斥锁，保证在任意时刻，只有一个线程可以访问数据
}

impl DBState {
    pub fn new(folder: &str) -> DBState {
        let conn = Connection::open(String::from(folder).add("/mx.db")).unwrap();
        Self {
            conn: Mutex::new(conn),
        }
    }
}
