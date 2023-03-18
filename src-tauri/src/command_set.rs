use std::sync::Mutex;

use crate::cipher_manage::cipher::Cipher;
use crate::cipher_manage::db_op::select::SelectList;
use crate::cipher_manage::db_op::DBOC;
use crate::db_opt::DBC;

pub struct DbConn {
    pub db: Mutex<DBC>,
    pub tb: Mutex<DBOC>,
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
pub fn upt_cipher(
    conn: tauri::State<DbConn>,
    uid: &str,
    station: &str,
    username: &str,
    pwd: &str,
    desc: &str,
) -> bool {
    let cipher = Cipher {
        uid: uid.to_string(),
        station: station.to_string(),
        username: username.to_string(),
        password: pwd.to_string(),
        desc: desc.to_string(),
    };
    conn.db.lock().unwrap().update_data(cipher).is_ok()
}

#[tauri::command]
pub fn get_all(conn: tauri::State<DbConn>) -> Vec<Cipher> {
    conn.db.lock().unwrap().get_all_data()
}

// new api
// 添加
#[tauri::command]
pub fn add_pwd(
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
    conn.tb.lock().unwrap().insert_cipher(cipher)
}

// 密码列表
#[tauri::command]
pub fn pwd_list(conn: tauri::State<DbConn>) -> Vec<SelectList> {
    conn.tb.lock().unwrap().get_cipher_list()
}

// 密码详情
#[tauri::command]
pub fn pwd_detail(conn: tauri::State<DbConn>, uid: &str) -> Result<Cipher, &'static str> {
    conn.tb.lock().unwrap().get_cipher_detail(uid)
}

// 删除密码
#[tauri::command]
pub fn del_pwd(conn: tauri::State<DbConn>, uid: &str) -> bool {
    conn.tb.lock().unwrap().delete_cipher(uid)
}
