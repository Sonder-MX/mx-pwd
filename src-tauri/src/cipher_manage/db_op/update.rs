use super::DBOC;
use crate::cipher_manage::{cipher::Cipher, TB_NAME};

impl DBOC {
    // 更新密码
    pub fn update_cipher(&self, cipher: Cipher) -> bool {
        self.conn
            .execute(
                "UPDATE ? SET station = ?, username = ?, password = ?, desc = ? WHERE uid = ?",
                (
                    TB_NAME,
                    cipher.station,
                    cipher.username,
                    cipher.password,
                    cipher.desc,
                    cipher.uid,
                ),
            )
            .is_ok()
    }
}
