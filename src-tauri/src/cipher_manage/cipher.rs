// use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
// use base64;
// use generic_array::GenericArray;
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

// AES加密
// pub fn encrypt_aes(plain_text: &str, key: &str) -> String {
//     let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_bytes()));
//     let nonce = GenericArray::from_slice(b"unique nonce");
//     let cipher_text = cipher
//         .encrypt(nonce, plain_text.as_bytes().into())
//         .expect("encryption failure!");
//     base64::encode(cipher_text)
// }

// AES解密
// pub fn decrypt_aes(cipher_text: &str, key: &str) -> String {
//     let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_bytes()));
//     let nonce = GenericArray::from_slice(b"unique nonce"); // 12 bytes; unique per message
//     let cipher_text = base64::decode(cipher_text).unwrap();
//     let plain_text = cipher
//         .decrypt(nonce, cipher_text.as_slice().into())
//         .expect("decryption failure!");
//     String::from_utf8(plain_text).unwrap()
// }
