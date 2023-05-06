pub mod cipher;

pub trait Models {
    const TB_NAME: &'static str;
    const FIELDS: &'static [&'static str];
    const FIELDS_TYPE: &'static [&'static str];

    // fn create_table(&self, db: &rusqlite::Connection, types: &[&str]) -> rusqlite::Result<()>;
    // fn insert(&self, db: &rusqlite::Connection, values: &[&str]) -> rusqlite::Result<()>;
    // fn delete(&self, db: &rusqlite::Connection, values: &[&str]) -> rusqlite::Result<()>;
    // fn update(&self, db: &rusqlite::Connection, values: &[&str]) -> rusqlite::Result<()>;
    // fn select(&self, db: &rusqlite::Connection, values: &[&str]) -> rusqlite::Result<()>;
}
