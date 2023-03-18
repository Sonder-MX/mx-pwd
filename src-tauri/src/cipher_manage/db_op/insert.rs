use super::DBOC;
use crate::cipher_manage::{cipher::Cipher, TB_NAME};

impl DBOC {
    // 添加
    pub fn insert_cipher(&self, cipher: Cipher) -> bool {
        self.conn
            .execute(
                "INSERT INTO ? (uid, station, username, password, desc) VALUES (?, ?, ?, ?, ?)",
                (
                    TB_NAME,
                    cipher.uid,
                    cipher.station,
                    cipher.username,
                    cipher.password,
                    cipher.desc,
                ),
            )
            .is_ok()
    }
}
