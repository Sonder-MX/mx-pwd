use std::collections::HashMap;

use super::{Cipher, FIELDS, FIELDS_TYPE, TB_NAME};
use crate::models::Models;
use crate::DBState;
use chrono::Local;
use nanoid::nanoid;
use rusqlite::{Connection, Result};

impl Models for Cipher {
    const TB_NAME: &'static str = TB_NAME;
    const FIELDS: &'static [&'static str] = FIELDS;
    const FIELDS_TYPE: &'static [&'static str] = FIELDS_TYPE;
}

impl Cipher {
    // utils for cipher
    // get now time
    pub fn get_now_time() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    // new cipher
    pub fn new(website: &str, username: &str, password: &str, remark: &str) -> Self {
        Self {
            nid: nanoid!(),
            website: website.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            remark: remark.to_string(),
            created: Self::get_now_time(),
            updated: Self::get_now_time(),
        }
    }

    // list
    pub fn overview(conn: &Connection) -> Vec<HashMap<String, String>> {
        let mut vlist = Vec::new();
        let mut stmt = conn
            .prepare(
                format!(
                    "SELECT {}, {}, {} FROM {}",
                    FIELDS[0], FIELDS[1], FIELDS[2], TB_NAME
                )
                .as_str(),
            )
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                let mut map = HashMap::new();
                map.insert(FIELDS[0].to_string(), row.get(0).unwrap()); // nid
                map.insert(FIELDS[1].to_string(), row.get(1).unwrap()); // website
                map.insert(FIELDS[2].to_string(), row.get(2).unwrap()); // username
                Ok(map)
            })
            .unwrap();
        for row in rows {
            // 按创建时间倒序
            vlist.insert(0, row.unwrap());
        }
        vlist
    }

    // detail
    pub fn detail(conn: &Connection, dnid: &str) -> Option<Result<Cipher>> {
        let mut stmt = conn
            .prepare(&format!("SELECT * FROM {} WHERE nid = '{}'", TB_NAME, dnid))
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok(Cipher {
                    nid: row.get(0)?,
                    website: row.get(1)?,
                    username: row.get(2)?,
                    password: row.get(3)?,
                    remark: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            })
            .unwrap();
        rows.last()
    }

    // add cipher
    pub fn adder(db: &DBState, cipher: Cipher) -> usize {
        let ins_fv = HashMap::from([
            (FIELDS[0], &cipher.nid as &str),
            (FIELDS[1], &cipher.website as &str),
            (FIELDS[2], &cipher.username as &str),
            (FIELDS[3], &cipher.password as &str),
            (FIELDS[4], &cipher.remark as &str),
            (FIELDS[5], &cipher.created as &str),
            (FIELDS[6], &cipher.updated as &str),
        ]);
        db.insert(TB_NAME, &ins_fv).unwrap()
    }

    // delete cipher
    pub fn deleter(db: &DBState, pk: &str) -> usize {
        let pk_fv = HashMap::from([(FIELDS[0], pk)]);
        db.delete(TB_NAME, &pk_fv).unwrap()
    }

    // update cipher
    pub fn updater(db: &DBState, pk: &str, set_fv: &HashMap<&str, &str>) -> usize {
        let pk_fv = HashMap::from([(FIELDS[0], pk)]);
        db.update(TB_NAME, set_fv, &pk_fv).unwrap()
    }
}
