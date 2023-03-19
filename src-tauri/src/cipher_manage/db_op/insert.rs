use super::DBOC;
use crate::cipher_manage::{cipher::Cipher, FIELD, TB_NAME};

impl DBOC {
    // 添加
    pub fn insert_cipher(&self, cipher: Cipher) -> bool {
        self.conn
            .execute(
                &format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    TB_NAME,
                    FIELD.join(","),
                    FIELD.iter().map(|_| "?").collect::<Vec<&str>>().join(",")
                ),
                (
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
