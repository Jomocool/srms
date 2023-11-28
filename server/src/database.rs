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

    /// 插入数据
    ///
    /// 参数：
    /// `table` - 表名
    /// `columns` - 列
    /// `values` - 值
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

    /// 删除数据
    ///
    /// 参数：
    /// `table` - 表名
    /// `where_columns` - 删除条件列名
    /// `where_values` - 删除条件对应的值
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

    /// 更新数据
    ///
    /// 参数：
    /// `table` - 表名
    /// `set_columns` - 需要更新的列名
    /// `set_values` - 对应的更新值
    /// `where_columns` - 更新条件列名
    /// `where_values` - 更新条件对应的值
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

    /// 查询数据
    ///
    /// 参数：
    /// `table` - 表名
    /// `select_columns` - 需要查询的列名，如果为空则查询所有列
    /// `where_columns` - 查询条件列名
    /// `where_values` - 查询条件对应的值
    ///
    /// 返回值：
    /// Vec<Row>：查找到的所有行
    pub fn select<T>(
        &mut self,
        table: &str,
        select_columns: Vec<&str>,
        where_columns: Vec<&str>,
        where_values: Vec<T>,
    ) -> Vec<Row>
    where
        T: mysql::prelude::ToValue,
    {
        let select_columns_str = if select_columns.is_empty() {
            "*".to_string()
        } else {
            select_columns.join(",")
        };

        let query = format!(
            "SELECT {} FROM {} {}",
            select_columns_str,
            table,
            if !where_columns.is_empty() {
                format!(
                    " WHERE {}",
                    where_columns
                        .iter()
                        .enumerate()
                        .map(|(i, column)| format!(
                            "{} = {}",
                            column,
                            where_values.get(i).unwrap().to_value().as_sql(true)
                        ))
                        .collect::<Vec<String>>()
                        .join(" AND ")
                )
            } else {
                "".to_string()
            }
        );

        let res = self.conn.query_iter(query);

        let rows = res.unwrap().collect::<Result<Vec<_>, _>>().unwrap();

        return rows;
    }
}
