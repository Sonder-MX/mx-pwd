pub mod cipher;

use std::{fs, path::Path};

use rusqlite::Connection;
use std::sync::Mutex;

pub struct DBState {
    pub conn: Mutex<Connection>,
}

impl Default for DBState {
    fn default() -> Self {
        if !Path::new("./user_data").exists() {
            fs::create_dir("./user_data").unwrap();
        }
        let conn = Connection::open("./user_data/data.db3").unwrap();
        Self {
            conn: Mutex::new(conn),
        }
    }
}

impl DBState {
    pub fn create_cipher_tb(&self) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "
        create table if not exists cipher (
            nid varchar(64) primary key,
            website varchar(128) not null,
            username varchar(64) not null,
            password varchar(128) not null,
            remark text,
            created datetime,
            updated datetime
        )
        ",
            (),
        )
        .unwrap();
    }
}
