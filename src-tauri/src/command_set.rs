use crate::pmanage::operation::Cipher;

#[tauri::command]
pub fn add_pwd(station: &str, username: &str, password: &str, desc: &str) {
    let cipher = Cipher::new(
        station.to_string(),
        username.to_string(),
        password.to_string(),
        Some(desc.to_string()),
    );
    match cipher.insert_data() {
        Ok(val) => println!("{} rows affected", val),
        Err(e) => println!("Error: {}", e),
    }
}

#[tauri::command]
pub fn get_all() -> Vec<Cipher> {
    Cipher::select_all()
}
