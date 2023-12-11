use mysql::{prelude::Queryable, Pool, PooledConn};

pub enum DBError {
    InsertError(String),
    UpdateError(String),
    DeleteError(String),
    SelectError(String),
}

pub struct DBHandler {
    conn: PooledConn,
}

impl DBHandler {
    pub fn new() -> DBHandler {
        let url = "mysql://jomo:12onetwo@192.168.168.132:3306/SRDB";
        let pool = Pool::new(url).unwrap();
        let conn = pool.get_conn().unwrap();
        return DBHandler { conn };
    }

    pub fn insert(&mut self, table: String, values: String) -> Result<String, DBError> {
        let query = format!("INSERT INTO {} VALUES ({})", table, values);
        let res = self.conn.query_iter(query);

        if res.is_err() {
            let errmsg = "添加失败！请重新检查添加信息";
            return Err(DBError::InsertError(errmsg.to_string()));
        }

        return Ok("添加成功！".to_string());
    }

    pub fn delete(&mut self, table: String, where_clause: String) -> Result<String, DBError> {
        let query = format!("DELETE FROM {} {}", table, where_clause);

        let res = self.conn.query_iter(query);

        if res.is_err() {
            let errmsg = "删除失败！请重新检查添加信息";
            return Err(DBError::DeleteError(errmsg.to_string()));
        }

        return Ok("删除成功！".to_string());
    }

    pub fn update(
        &mut self,
        table: String,
        set_clause: String,
        where_clause: String,
    ) -> Result<String, DBError> {
        let query = format!("UPDATE {} SET {} {}", table, set_clause, where_clause,);

        let res = self.conn.query_iter(query);

        if res.is_err() {
            let errmsg = "更新失败！请重新检查添加信息";
            return Err(DBError::UpdateError(errmsg.to_string()));
        }

        return Ok("更新成功！".to_string());
    }

    pub fn select(
        &mut self,
        table: String,
        select_columns: String,
        where_clause: String,
    ) -> Result<String, DBError> {
        let query = format!("SELECT {} FROM {} {}", select_columns, table, where_clause,);

        let res = self.conn.query_iter(query);

        if res.is_err() {
            let errmsg = "查询失败！请检查查询信息";
            return Err(DBError::SelectError(errmsg.to_string()));
        }

        let message: String = {
            let rows = res.unwrap().collect::<Result<Vec<_>, _>>().unwrap();
            let mut response_body = String::new();
            for row in rows {
                // 遍历每个字段
                let mut row_str = String::new();
                for i in 0..row.len() {
                    let col_val = format!(
                        "| {}: {} ",
                        &row.columns()[i].name_str(),
                        row.get::<String, _>(i).unwrap()
                    );
                    row_str.push_str(&col_val);
                }
                row_str.push_str("\n");
                response_body.push_str(&row_str);
            }
            response_body.push('\n');
            response_body
        };

        return Ok(message);
    }
}
