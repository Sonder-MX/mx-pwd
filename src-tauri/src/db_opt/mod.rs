use rusqlite::{Connection, Result};
use std::{fs, path::Path};

pub fn db_exists() -> Result<Connection> {
    let user_folder = Path::new("./userdata");
    if !user_folder.exists() {
        fs::create_dir(user_folder).expect("Failed to create userdata folder");
    }
    let conn = Connection::open("./userdata/db.sqlite3")?;
    Ok(conn)
}

pub fn create_tb(conn: &Connection, tb_name: &str, field: Vec<&str>, attr: Vec<&str>) {
    let mut content = String::new();
    for (i, j) in field.iter().zip(attr.iter()) {
        content.push_str(&format!("{} {}, ", i, j));
    }
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            tb_name,
            content.trim_end_matches(", ")
        ),
        (),
    )
    .unwrap();
}
