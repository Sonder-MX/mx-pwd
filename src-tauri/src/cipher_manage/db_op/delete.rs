use super::DBOC;
use crate::cipher_manage::TB_NAME;

impl DBOC {
    // 删除密码
    pub fn delete_cipher(&self, uid: &str) -> bool {
        self.conn
            .execute("DELETE FROM ? WHERE uid = ?", (TB_NAME, uid))
            .is_ok()
    }
}
