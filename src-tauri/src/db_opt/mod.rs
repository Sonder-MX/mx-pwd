use rusqlite::{params, Connection, Result};

use crate::cipher_manage::{cipher::Cipher, ATTR, FIELD, TB_NAME};

const DB_FILE: &str = "./userdata/db.sqlite3";

pub struct DBC {
    pub conn: Connection,
}

impl DBC {
    pub fn new() -> DBC {
        let (tb_name, field, attr) = (TB_NAME, FIELD, ATTR);
        let conn = Connection::open(DB_FILE).unwrap();
        let mut statement = String::new();
        for (i, j) in field.iter().zip(attr.iter()) {
            statement.push_str(&format!("{i} {j}, "));
        }
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                tb_name,
                statement.trim_end_matches(", ")
            ),
            (),
        )
        .unwrap();
        DBC { conn }
    }

    pub fn insert_data(&self, cipher: Cipher) -> Result<usize> {
        self.conn.execute(
            &format!(
                "INSERT INTO {} ({}) VALUES ({})",
                TB_NAME,
                FIELD.join(","),
                FIELD.iter().map(|_| "?").collect::<Vec<&str>>().join(",")
            ),
            params![
                cipher.uid,
                cipher.station,
                cipher.username,
                cipher.password,
                cipher.desc
            ],
        )
    }

    pub fn get_all_data(&self) -> Vec<Cipher> {
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT * FROM {}", TB_NAME))
            .unwrap();
        let mut data = Vec::new();
        let cipher_iter = stmt.query_map([], |row| {
            Ok(Cipher {
                uid: row.get(0)?,
                station: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                desc: row.get(4)?,
            })
        });
        for c in cipher_iter.unwrap() {
            data.push(c.unwrap());
        }
        data
    }
}
