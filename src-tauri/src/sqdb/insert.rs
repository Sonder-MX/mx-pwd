use super::DBState;
use rusqlite::Result;

use std::collections::HashMap;

impl DBState {
    /// 向表中插入记录
    /// - tb: 表名
    /// - ins_fv: 插入字段值 (字段名, 字段值)
    /// - Ok(usize): 插入的记录数
    /// - Err(rusqlite::Error): 错误信息
    pub fn insert(&self, tb: &str, ins_fv: &HashMap<&str, &str>) -> Result<usize> {
        let stmt = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            tb,
            ins_fv
                .iter()
                .map(|(f, _)| format!("'{}'", f))
                .collect::<Vec<String>>()
                .join(", "),
            ins_fv
                .iter()
                .map(|(_, v)| format!("'{}'", v))
                .collect::<Vec<String>>()
                .join(", ")
        );
        self.conn.lock().unwrap().execute(&stmt, ())
    }
}
