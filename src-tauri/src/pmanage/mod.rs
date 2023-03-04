use crate::db_opt;
use rusqlite::Connection;

pub mod operation;

const TB_NAME: &str = "pmanage";
const FIELD: [&str; 5] = ["uid", "station", "username", "password", "desc"];
const ATTR: [&str; 5] = [
    "TEXT PRIMARY KEY",
    "TEXT NOT NULL",
    "TEXT NULL",
    "TEXT NULL",
    "TEXT",
];

pub fn pmanage_tb() -> Connection {
    let conn = db_opt::db_exists().unwrap();
    db_opt::create_tb(&conn, TB_NAME, FIELD.to_vec(), ATTR.to_vec());
    conn
}
