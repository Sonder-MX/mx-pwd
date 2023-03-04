use nanoid::nanoid;
use rusqlite::{params, Result};

use super::*;

#[derive(serde::Serialize)]
pub struct Cipher {
    pub uid: String,
    pub station: String,
    pub username: String,
    pub password: String,
    pub desc: Option<String>,
}

impl Cipher {
    pub fn new(
        station: String,
        username: String,
        password: String,
        desc: Option<String>,
    ) -> Cipher {
        Cipher {
            uid: nanoid!(),
            station,
            username,
            password,
            desc,
        }
    }

    pub fn insert_data(&self) -> Result<usize> {
        let conn = pmanage_tb();
        conn.execute(
            &format!(
                "INSERT INTO {} (uid, station, username, password, desc) VALUES (?1, ?2, ?3, ?4, ?5)",
                TB_NAME
            ),
            params![
                self.uid,
                self.station,
                self.username,
                self.password,
                self.desc
            ],
        )
    }

    // 查看所有数据
    pub fn select_all() -> Vec<Cipher> {
        let conn = pmanage_tb();
        let mut stmt = conn.prepare(&format!("SELECT * FROM {}", TB_NAME)).unwrap();
        let mut ciphers = Vec::new();
        let cipher_iter = stmt.query_map([], |row| {
            Ok(Cipher {
                uid: row.get(0)?,
                station: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                desc: row.get(4)?,
            })
        });
        for cipher in cipher_iter.unwrap() {
            ciphers.push(cipher.unwrap());
        }
        ciphers
    }
}
