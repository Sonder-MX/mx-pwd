use super::DBOC;
use crate::cipher_manage::cipher::Cipher;
use crate::cipher_manage::TB_NAME;

#[derive(serde::Serialize)]
pub struct SelectList {
    pub uid: String,
    pub station: String,
    pub username: String,
}

impl DBOC {
    // 密码列表
    pub fn get_cipher_list(&self) -> Vec<SelectList> {
        let mut statement = self
            .conn
            .prepare(&format!("SELECT uid, station, username FROM {TB_NAME}"))
            .unwrap();
        let mut cipher_list = Vec::new();
        let cipher_iter = statement.query_map((), |row| {
            Ok(SelectList {
                uid: row.get(0)?,
                station: row.get(1)?,
                username: row.get(2)?,
            })
        });
        for c in cipher_iter.unwrap() {
            cipher_list.push(c.unwrap());
        }
        cipher_list
    }

    // 密码详情
    pub fn get_cipher_detail(&self, uid: &str) -> Result<Cipher, &'static str> {
        let mut statement = self
            .conn
            .prepare(&format!("SELECT * FROM {TB_NAME} WHERE uid = {uid}"))
            .unwrap();
        let cipher_row = statement.query_map((), |row| {
            Ok(Cipher {
                uid: row.get(0)?,
                station: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                desc: row.get(4)?,
            })
        });
        let cipher = cipher_row.unwrap().next().unwrap();
        match cipher {
            Ok(data) => Ok(data),
            Err(_) => Err("No Data"),
        }
    }
}
