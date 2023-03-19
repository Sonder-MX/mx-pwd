use super::DBOC;
use crate::cipher_manage::{cipher::Cipher, TB_NAME};

use rusqlite::params;

impl DBOC {
    // 更新密码
    pub fn update_cipher(&self, cipher: Cipher) -> bool {
        self.conn
            .execute(
                &format!(
                    "UPDATE {TB_NAME} SET station = ?, username = ?, password = ?, desc = ? WHERE uid = ?"
                ),
                params![
                    cipher.station,
                    cipher.username,
                    cipher.password,
                    cipher.desc,
                    cipher.uid,
                ],
            ).is_ok()
    }
}
