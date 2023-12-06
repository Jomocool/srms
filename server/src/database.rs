// use chrono::NaiveDate;
use mysql::{prelude::Queryable, Pool, PooledConn, Row, Value};

// struct WorkPlace {
//     id: Option<i32>,
//     area: Option<u32>,
//     address: Option<String>,
// }

// struct Lab {
//     id: Option<i32>,
//     name: Option<String>,
//     introduction: Option<String>,
//     work_place_id: Option<i32>,
// }

// struct Researcher {
//     lab_id: Option<i32>,
//     id: Option<i32>,
//     name: Option<String>,
//     gender: Option<String>,
//     title: Option<String>,
//     age: Option<i32>,
//     research_direction: Option<String>,
// }

// struct Director {
//     researcher_id: Option<i32>,
//     start_date: Option<NaiveDate>,
//     term: Option<i32>,
// }

// struct Secretary {
//     lab_id: Option<i32>,
//     id: Option<i32>,
//     name: Option<String>,
//     gender: Option<String>,
//     age: Option<i32>,
//     start_date: Option<NaiveDate>,
//     duty: Option<String>,
// }

// struct ResearchProject {
//     id: Option<i32>,
//     leader_id: Option<i32>,
//     name: Option<String>,
//     content: Option<String>,
//     funding: Option<f64>,
//     start_date: Option<NaiveDate>,
//     end_date: Option<NaiveDate>,
// }

// struct ProjectClient {
//     id: Option<i32>,
//     project_id: Option<i32>,
//     name: Option<String>,
//     address: Option<String>,
//     leader_name: Option<String>,
//     leader_telephone: Option<String>,
//     leader_phone: Option<String>,
//     leader_email: Option<String>,
//     contact_name: Option<String>,
//     contact_telephone: Option<String>,
//     contact_phone: Option<String>,
//     contact_email: Option<String>,
// }

// struct ProjectCollaborator {
//     id: Option<i32>,
//     project_id: Option<i32>,
//     name: Option<String>,
//     address: Option<String>,
//     leader_name: Option<String>,
//     leader_telephone: Option<String>,
//     leader_phone: Option<String>,
//     leader_email: Option<String>,
//     contact_name: Option<String>,
//     contact_telephone: Option<String>,
//     contact_phone: Option<String>,
//     contact_email: Option<String>,
// }

// struct QualityMonitor {
//     id: Option<i32>,
//     project_id: Option<i32>,
//     name: Option<String>,
//     address: Option<String>,
//     leader_name: Option<String>,
//     leader_telephone: Option<String>,
//     leader_phone: Option<String>,
//     leader_email: Option<String>,
//     contact_name: Option<String>,
//     contact_telephone: Option<String>,
//     contact_phone: Option<String>,
//     contact_email: Option<String>,
// }

// struct ProjectParticipant {
//     project_id: Option<i32>,
//     researcher_id: Option<i32>,
//     join_date: Option<NaiveDate>,
//     workload: Option<i32>,
//     disposable_funds: Option<f64>,
// }

// struct ProjectSubtask {
//     project_id: Option<i32>,
//     leader_id: Option<i32>,
//     subtask_num: Option<i32>,
//     deadline: Option<NaiveDate>,
//     disposable_funds: Option<f64>,
//     technical_indicators: Option<String>,
// }

// struct ResearchOutcome {
//     id: Option<i32>,
//     project_id: Option<i32>,
//     name: Option<String>,
//     achieve_date: Option<NaiveDate>,
//     rank_num: Option<i32>,
// }

// struct OutcomeContributor {
//     id: Option<i32>,
//     outcome_id: Option<i32>,
//     researcher_id: Option<i32>,
// }

// struct Patent {
//     id: Option<i32>,
//     outcome_id: Option<i32>,
//     paten_type: Option<String>,
// }

// struct Paper {
//     id: Option<i32>,
//     outcome_id: Option<i32>,
// }

// struct SoftwareCopyright {
//     id: Option<i32>,
//     outcome_id: Option<i32>,
// }

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

    pub fn insert<T>(
        &mut self,
        table: &str,
        columns: Vec<&str>,
        values: Vec<T>,
    ) -> Result<(), mysql::Error>
    where
        T: mysql::prelude::ToValue,
    {
        let columns_str = columns.join(",");
        let values_str = values.iter().map(|_| "?").collect::<Vec<&str>>().join(",");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table, columns_str, values_str
        );

        let values: Vec<Value> = values.into_iter().map(|v| v.to_value()).collect();
        self.conn.exec_drop(query, values)?;

        return Ok(());
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

    pub fn update<T>(
        &mut self,
        table: &str,
        set_columns: Vec<&str>,
        set_values: Vec<T>,
        where_columns: Vec<&str>,
        where_values: Vec<T>,
    ) -> Result<(), mysql::Error>
    where
        T: mysql::prelude::ToValue,
    {
        let query = format!(
            "UPDATE {} SET {} WHERE {}",
            table,
            set_columns
                .iter()
                .enumerate()
                .map(|(_, column)| format!("{} = ?", column))
                .collect::<Vec<String>>()
                .join(", "),
            where_columns
                .iter()
                .enumerate()
                .map(|(_, column)| format!("{} = ?", column))
                .collect::<Vec<String>>()
                .join(" AND ")
        );

        let mut params: Vec<Value> = Vec::new();
        params.extend(set_values.into_iter().map(|v| v.to_value()));
        params.extend(where_values.into_iter().map(|v| v.to_value()));

        self.conn.exec_drop(query, params)?;

        return Ok(());
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
                response_body.push_str("------------------------------------------\n");
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
                    response_body.push_str("------------------------------------------\n");
                }
                response_body.push('\n');
                response_body
            }
            Err(_) => "查询出错！".to_string(),
        };

        return message;
    }
}
