pub mod delete;
pub mod select;
pub mod update;

use rusqlite::Connection;

use super::{ATTR, FIELD, TB_NAME};

pub struct DBOC {
    pub conn: Connection,
}

impl DBOC {
    pub fn new(db_file: &str) -> DBOC {
        let (tb_name, field, attr) = (TB_NAME, FIELD, ATTR);
        let conn = Connection::open(db_file).unwrap();
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
        DBOC { conn }
    }
}
