use super::DBState;
use rusqlite::Result;

use std::collections::HashMap;

impl DBState {
    /// 从表中删除记录
    /// - tb: 表名
    /// - del_fv: 删除条件 (字段名, 字段值)
    /// - Ok(usize): 删除的记录数
    /// - Err(rusqlite::Error): 错误信息
    pub fn delete(&self, tb: &str, del_fv: &HashMap<&str, &str>) -> Result<usize> {
        let stmt = format!(
            "DELETE FROM {} WHERE {}",
            tb,
            del_fv
                .iter()
                .map(|(f, v)| format!("{} = {}", f, v))
                .collect::<Vec<String>>()
                .join(" AND ")
        );
        self.conn.lock().unwrap().execute(&stmt, ())
    }
}
