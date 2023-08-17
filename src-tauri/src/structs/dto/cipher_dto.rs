use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CipherDTO {
    id: Option<i128>,
    website: Option<String>,
    account: Option<String>,
    password: Option<String>,
    notes: Option<String>,
    create_time: Option<NaiveDateTime>,
    update_time: Option<NaiveDateTime>,
}
