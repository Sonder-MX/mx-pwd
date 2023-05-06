use std::collections::HashMap;

use crate::Cipher;
use crate::DBState;

#[tauri::command]
pub fn get_cipher_list(db_state: tauri::State<'_, DBState>) -> Vec<HashMap<String, String>> {
    let conn = db_state.conn.lock().unwrap();
    Cipher::overview(&conn)
}

#[tauri::command]
pub fn get_cipher_detail(
    db_state: tauri::State<'_, DBState>,
    dnid: &str,
) -> Result<Vec<Cipher>, String> {
    let conn = db_state.conn.lock().unwrap();
    let cipher_detail = Cipher::detail(&conn, dnid).unwrap();
    if cipher_detail.len() == 0 {
        return Err("Cipher not found!".into());
    }
    Ok(cipher_detail)
}
