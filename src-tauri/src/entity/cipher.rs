use chrono::prelude::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cipher {
    pub nid: String,
    pub website: String,
    pub username: String,
    pub password: String,
    pub remark: Option<String>,
    #[serde(serialize_with = "serailizer_time")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serailizer_time")]
    pub updated: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCipher {
    pub nid: Option<String>,
    pub website: String,
    pub username: String,
    pub password: String,
    pub remark: Option<String>,
}

fn serailizer_time<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
}
