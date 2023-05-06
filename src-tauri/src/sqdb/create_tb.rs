use super::DBState;

impl DBState {
    pub fn create_table(&self, tb_name: &str, fields: &[&str], ft: &[&str]) {
        let mut sql = String::from("CREATE TABLE IF NOT EXISTS ");
        sql.push_str(tb_name);
        sql.push_str(" (");
        for (i, field) in fields.iter().enumerate() {
            sql.push_str(field);
            sql.push_str(" ");
            sql.push_str(ft[i]);
            sql.push_str(", ");
        }
        sql.pop();
        sql.pop();
        sql.push_str(");");
        self.conn
            .lock()
            .unwrap()
            .execute(&sql, [])
            .expect("Failed to create table!");
    }
}
