#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod invokes;
mod models;
mod sqdb;
mod structs;

use std::{fs, path::Path};

use invokes::cipher_invoke::*;
use models::cipher::Cipher;
use models::Models;
use sqdb::DBState;

fn main() {
    let user_folder = "./user";
    if !Path::new(user_folder).exists() {
        fs::create_dir("./user").expect("Failed to create user folder!");
    }
    let db_state = DBState::new(user_folder);
    db_state.create_table(Cipher::TB_NAME, Cipher::FIELDS, Cipher::FIELDS_TYPE); // create cipher table

    tauri::Builder::default()
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            get_cipher_list,
            get_cipher_detail,
            add_cipher,
            delete_cipher,
            update_cipher
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
