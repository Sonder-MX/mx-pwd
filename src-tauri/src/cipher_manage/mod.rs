pub mod cipher;

pub const TB_NAME: &str = "Cipher";
pub const FIELD: [&str; 5] = ["uid", "station", "username", "password", "desc"];
pub const ATTR: [&str; 5] = [
    "TEXT PRIMARY KEY",
    "TEXT NOT NULL",
    "TEXT NULL",
    "TEXT NULL",
    "TEXT",
];
