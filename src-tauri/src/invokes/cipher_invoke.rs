use chrono::prelude::Local;
use nanoid::nanoid;
use rusqlite::params;
use tauri::State;

use super::Response;
use crate::entity::cipher::{Cipher, CreateCipher};
use crate::entity::DBState;
use crate::utils::constant::*;

#[tauri::command]
pub fn add_cipher(db_state: State<'_, DBState>, mut cipher: CreateCipher) -> Response<u8> {
    let conn = db_state.conn.lock().unwrap();

    cipher.nid = Some(nanoid!());
    let date_time = Local::now().naive_local();

    let row = conn.execute(
        "insert into cipher (nid, website, username, password, remark, created, updated) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![cipher.nid, cipher.website, cipher.username, cipher.password, cipher.remark, date_time, date_time],
    );

    if row.is_err() {
        Response::error(msg_const::ADD_FAIL)
    } else {
        Response::success(Some(msg_const::ADD_SUCCESS.to_string()))
    }
}

#[tauri::command]
pub fn query_cipher(db_state: State<'_, DBState>) -> Response<Vec<Cipher>> {
    let conn = db_state.conn.lock().unwrap();

    let mut stmt = conn.prepare("select * from cipher").unwrap();

    let cipher_iter = stmt
        .query_map(params![], |row| {
            Ok(Cipher {
                nid: row.get(0)?,
                website: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                remark: row.get(4)?,
                created: row.get(5)?,
                updated: row.get(6)?,
            })
        })
        .unwrap();

    let mut ciphers = Vec::new();
    for cipher in cipher_iter {
        ciphers.push(cipher.unwrap());
    }
    Response::success_with_data(ciphers)
}

#[tauri::command]
pub fn delete_cipher(db_state: State<'_, DBState>, nid: String) -> Response<u8> {
    let conn = db_state.conn.lock().unwrap();

    let row = conn.execute("delete from cipher where nid = ?1", params![nid]);

    if row.is_err() {
        Response::error(msg_const::DELETE_FAIL)
    } else {
        Response::success(Some(msg_const::DELETE_SUCCESS.to_string()))
    }
}

#[tauri::command]
pub fn update_cipher(db_state: State<'_, DBState>, mut cipher: Cipher) -> Response<u8> {
    let conn = db_state.conn.lock().unwrap();

    cipher.updated = Local::now().naive_local();
    let row = conn.execute(
        "update cipher set website = ?1, username = ?2, password = ?3, remark = ?4, updated = ?5 where nid = ?6",
        params![cipher.website, cipher.username, cipher.password, cipher.remark, cipher.updated, cipher.nid],
    );

    if row.is_err() {
        Response::error(msg_const::UPDATE_FAIL)
    } else {
        Response::success(Some(msg_const::UPDATE_SUCCESS.to_string()))
    }
}

#[tauri::command]
pub fn query_cipher_by_nid(db_state: State<'_, DBState>, nid: String) -> Response<Cipher> {
    let conn = db_state.conn.lock().unwrap();

    let mut stmt = conn.prepare("select * from cipher where nid = ?1").unwrap();

    let cipher_iter = stmt
        .query_map(params![nid], |row| {
            Ok(Cipher {
                nid: row.get(0)?,
                website: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                remark: row.get(4)?,
                created: row.get(5)?,
                updated: row.get(6)?,
            })
        })
        .unwrap();

    let mut ciphers = Vec::new();
    for cipher in cipher_iter {
        ciphers.push(cipher.unwrap());
    }
    if ciphers.len() > 0 {
        Response::success_with_data(ciphers[0].clone())
    } else {
        Response::error("查询失败")
    }
}
