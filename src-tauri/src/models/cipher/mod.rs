pub mod methods;
mod test;

use serde::Serialize;

pub const TB_NAME: &str = "cipher";
pub const FIELDS: &[&str] = &[
    "nid", "website", "username", "password", "remark", "created", "updated",
];
pub const FIELDS_TYPE: &[&str] = &[
    "TEXT NOT NULL PRIMARY KEY",
    "TEXT NOT NULL",
    "TEXT NOT NULL",
    "TEXT NOT NULL",
    "TEXT",
    "TEXT",
    "TEXT",
];

#[derive(Debug, Serialize)]
pub struct Cipher {
    pub nid: String,
    pub website: String,
    pub username: String,
    pub password: String,
    pub remark: String,
    pub created: String,
    pub updated: String,
}
