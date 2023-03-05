use std::sync::Mutex;

use crate::cipher_manage::cipher::Cipher;
use crate::db_opt::DBC;

pub struct DbConn {
    pub db: Mutex<DBC>,
}

#[tauri::command]
pub fn add_cipher(
    conn: tauri::State<DbConn>,
    station: &str,
    username: &str,
    pwd: &str,
    desc: &str,
) -> bool {
    let cipher = Cipher::new(
        station.to_string(),
        username.to_string(),
        pwd.to_string(),
        desc.to_string(),
    );
    conn.db.lock().unwrap().insert_data(cipher).is_ok()
}

#[tauri::command]
pub fn del_cipher(conn: tauri::State<DbConn>, uid: &str) -> bool {
    conn.db.lock().unwrap().delete_data(uid).is_ok()
}

#[tauri::command]
pub fn upt_cipher(conn: tauri::State<DbConn>, cipher: Cipher) -> bool {
    conn.db.lock().unwrap().update_data(cipher).is_ok()
}

#[tauri::command]
pub fn get_all(conn: tauri::State<DbConn>) -> Vec<Cipher> {
    conn.db.lock().unwrap().get_all_data()
}
