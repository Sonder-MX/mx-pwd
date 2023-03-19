use super::DBOC;
use crate::cipher_manage::TB_NAME;

impl DBOC {
    // 删除密码
    pub fn delete_cipher(&self, uid: &str) -> bool {
        self.conn
            .execute(&format!("DELETE FROM {TB_NAME} WHERE uid = ?"), [uid])
            .is_ok()
    }
}
