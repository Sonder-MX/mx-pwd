#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cipher_manage;
mod command_set;
mod db_opt;

use std::{fs, path::Path, sync::Mutex};

use command_set::*;
use db_opt::DBC;

fn main() {
    if !Path::new("./userdata").exists() {
        fs::create_dir("./userdata").unwrap();
    }

    let pwd_tb = DBC::new();

    tauri::Builder::default()
        .manage(DbConn {
            db: Mutex::from(pwd_tb),
        })
        .invoke_handler(tauri::generate_handler![
            add_cipher, del_cipher, upt_cipher, get_all
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
