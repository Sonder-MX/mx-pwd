use std::collections::HashMap;

use super::{Response, ResponseData};
use crate::models::Models;
use crate::Cipher;
use crate::DBState;
use serde_json::json;

#[tauri::command]
pub fn get_cipher_list(db_state: tauri::State<'_, DBState>) -> Vec<HashMap<String, String>> {
    let conn = db_state.conn.lock().unwrap();
    Cipher::overview(&conn)
}

#[tauri::command]
pub fn get_cipher_detail(
    db_state: tauri::State<'_, DBState>,
    dnid: &str,
) -> Result<Cipher, ResponseData> {
    let conn = db_state.conn.lock().unwrap();
    let cipher_detail = Cipher::detail(&conn, dnid);
    if cipher_detail.is_none() {
        Err(ResponseData::new(
            404,
            "Cipher detail not found!".into(),
            json!(null),
        ))
    } else {
        Ok(cipher_detail.unwrap().unwrap())
    }
}

#[tauri::command]
pub fn add_cipher(
    db_state: tauri::State<'_, DBState>,
    website: &str,
    username: &str,
    password: &str,
    remark: &str,
) -> Result<ResponseData, ResponseData> {
    let cipher = Cipher::new(website, username, password, remark);
    let res = Cipher::adder(&db_state, cipher);
    if res == 0 {
        Err(ResponseData::new(
            401,
            "Add cipher failed!".into(),
            json!({
                "col": res,
            }),
        ))
    } else {
        Ok(ResponseData::new(
            200,
            "Add cipher success!".into(),
            json!(
                {
                    "col": res,
                }
            ),
        ))
    }
}

#[tauri::command]
pub fn update_cipher(
    db_state: tauri::State<'_, DBState>,
    nid: &str,
    website: &str,
    username: &str,
    password: &str,
    remark: &str,
) -> Result<ResponseData, ResponseData> {
    let now = Cipher::get_now_time();
    let new_val = HashMap::from([
        (Cipher::FIELDS[1], website),
        (Cipher::FIELDS[2], username),
        (Cipher::FIELDS[3], password),
        (Cipher::FIELDS[4], remark),
        (Cipher::FIELDS[6], &now),
    ]);
    let res = Cipher::updater(&db_state, nid, &new_val);
    if res == 0 {
        Err(ResponseData::new(
            401,
            "Update cipher failed!".into(),
            json!({
                "col": res,
            }),
        ))
    } else {
        Ok(ResponseData::new(
            200,
            "Update cipher success!".into(),
            json!(
                {
                    "col": res,
                }
            ),
        ))
    }
}

#[tauri::command]
pub fn delete_cipher(db_state: tauri::State<'_, DBState>, nid: &str) -> Response<u8> {
    let res = Cipher::deleter(&db_state, nid);
    if res == 0 {
        Response::fail()
    } else {
        Response::success()
    }
}
