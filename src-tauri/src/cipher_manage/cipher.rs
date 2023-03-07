use nanoid::nanoid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Cipher {
    pub uid: String,
    pub station: String,
    pub username: String,
    pub password: String,
    pub desc: String,
}

impl Cipher {
    pub fn new(station: String, username: String, password: String, desc: String) -> Cipher {
        Cipher {
            uid: nanoid!(),
            station,
            username,
            password,
            desc,
        }
    }
}
