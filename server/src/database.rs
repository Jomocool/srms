// use chrono::NaiveDate;
use mysql::{prelude::Queryable, Pool, PooledConn, Value};
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

    pub fn insert(&mut self, table: String, values: String) -> String {
        let query = format!("INSERT INTO {} VALUES ({})", table, values);
        let res = self.conn.query_iter(query);

        if res.is_err() {
            return "添加失败！请重新检查添加信息".to_string();
        }

        return "添加成功！".to_string();
    }

    pub fn delete<T>(
        &mut self,
        table: &str,
        where_columns: Vec<&str>,
        where_values: Vec<T>,
    ) -> Result<(), mysql::Error>
    where
        T: mysql::prelude::ToValue,
    {
        let query = format!(
            "DELETE FROM {} WHERE {}",
            table,
            where_columns
                .iter()
                .enumerate()
                .map(|(_, column)| format!("{} = ?", column))
                .collect::<Vec<String>>()
                .join(" AND ")
        );

        let values: Vec<Value> = where_values.into_iter().map(|v| v.to_value()).collect();

        self.conn.exec_drop(query, values)?;

        return Ok(());
    }

    pub fn update(&mut self, table: String, set_clause: String, where_clause: String) -> String {
        let query = format!("UPDATE {} SET {} {}", table, set_clause, where_clause,);

        let res = self.conn.query_iter(query);

        if res.is_err() {
            return "更新失败！请重新检查添加信息".to_string();
        }

        return "更新成功！".to_string();
    }

    pub fn select(
        &mut self,
        table: String,
        select_columns: String,
        where_clause: String,
    ) -> String {
        let query = format!("SELECT {} FROM {} {}", select_columns, table, where_clause,);

        let res = self.conn.query_iter(query);

        let message: String = match res {
            Ok(res) => {
                let rows = res.collect::<Result<Vec<_>, _>>().unwrap();
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
            }
            Err(_) => "查询失败！请检查查询信息".to_string(),
        };

        return message;
    }
}
