use super::DBState;
use rusqlite::{params, Result};

use std::collections::HashMap;

impl DBState {
    /// 更新表中记录
    /// - tb: 表名
    /// - set_fv: 更新字段值 (字段名, 字段值)
    /// - pk_fv: 更新条件 (字段名, 字段值)
    /// - Ok(usize): 更新的记录数
    /// - Err(rusqlite::Error): 错误信息
    pub fn update(
        &self,
        tb: &str,
        set_fv: &HashMap<&str, &str>,
        pk_fv: &HashMap<&str, &str>,
    ) -> Result<usize> {
        let stmt = format!(
            "UPDATE {} SET {} WHERE {}",
            tb,
            set_fv
                .iter()
                .map(|(k, v)| format!("{} = '{}'", k, v))
                .collect::<Vec<String>>()
                .join(", "),
            pk_fv
                .iter()
                .map(|(k, v)| format!("{} = '{}'", k, v))
                .collect::<Vec<String>>()
                .join(" AND "),
        );
        self.conn.lock().unwrap().execute(&stmt, params![])
    }
}
