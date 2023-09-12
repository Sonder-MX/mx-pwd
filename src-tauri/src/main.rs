#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod entity;
mod invokes;
mod utils;

use entity::DBState;
use invokes::cipher_invoke::*;

fn main() {
    let db_state = DBState::default();
    db_state.create_cipher_tb();

    tauri::Builder::default()
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            add_cipher,
            query_cipher,
            delete_cipher,
            update_cipher,
            query_cipher_by_nid
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
