use super::{Cipher, FIELDS, FIELDS_TYPE, TB_NAME};
use crate::models::Models;
use rusqlite::{Connection, Result};

use std::collections::HashMap;

impl Models for Cipher {
    const TB_NAME: &'static str = TB_NAME;
    const FIELDS: &'static [&'static str] = FIELDS;
    const FIELDS_TYPE: &'static [&'static str] = FIELDS_TYPE;
}

impl Cipher {
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
                map.insert(FIELDS[0].to_string(), row.get(0).unwrap());
                map.insert(FIELDS[1].to_string(), row.get(1).unwrap());
                map.insert(FIELDS[2].to_string(), row.get(2).unwrap());
                Ok(map)
            })
            .unwrap();
        for row in rows {
            vlist.push(row.unwrap());
        }
        vlist
    }

    // detail
    pub fn detail(conn: &Connection, dnid: &str) -> Result<Vec<Cipher>> {
        let mut stmt =
            conn.prepare(&format!("SELECT * FROM {} WHERE nid = '{}'", TB_NAME, dnid))?;
        let rows = stmt.query_map([], |row| {
            Ok(Cipher {
                nid: row.get(0)?,
                website: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                remark: row.get(4)?,
                created: row.get(5)?,
                updated: row.get(6)?,
            })
        })?;
        let mut vlist = Vec::new();
        for row in rows {
            vlist.push(row.unwrap());
        }
        Ok(vlist)
    }
}
