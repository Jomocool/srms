use database::DBHandler;
use mysql::Value;

pub mod database;

fn main() {
    let mut srms_handler = DBHandler::new();
    let table = "WorkPlace";

    // 1. 插入数据到WorkPlace
    let columns = vec!["id", "area", "address"];
    let values = vec![Value::from(1), Value::from(100), Value::from("中国")];
    let res = srms_handler.insert(table, columns, values);
    if let Err(e) = res {
        println!("{}", e);
    } else {
        println!("插入成功！");
    }

    // 2. 查找WorkPlace所有数据
    let select_columns = vec![];
    let where_columns = vec![];
    let where_values: Vec<Value> = vec![];
    let rows = srms_handler.select("WorkPlace", select_columns, where_columns, where_values);
    for row in rows {
        // 遍历每个字段
        for i in 0..row.len() {
            // 打印字段名和值
            println!(
                "{}: {}",
                row.columns()[i].name_str(),
                row.get::<String, _>(i).unwrap()
            );
        }
        println!("---------------------");
    }

    // 3. 更新WorkPlace表中所有地区为中国的数据，将地址改为深圳
    let set_columns = vec!["address"];
    let set_values = vec![Value::from("深圳")];
    let where_columns = vec!["address"];
    let where_values = vec![Value::from("中国")];
    let res = srms_handler.update(table, set_columns, set_values, where_columns, where_values);
    if let Err(e) = res {
        println!("{}", e);
    } else {
        println!("更新成功！");
    }

    // 4. 查找WorkPlace更新后的数据
    let select_columns = vec![];
    let where_columns = vec![];
    let where_values: Vec<Value> = vec![];
    let rows = srms_handler.select("WorkPlace", select_columns, where_columns, where_values);
    for row in rows {
        // 遍历每个字段
        for i in 0..row.len() {
            // 打印字段名和值
            println!(
                "{}: {}",
                row.columns()[i].name_str(),
                row.get::<String, _>(i).unwrap()
            );
        }
        println!("---------------------");
    }

    // 5. 删掉WorkPlace中地址为深圳的数据
    let where_columns = vec!["address"];
    let where_values = vec![Value::from("深圳")];
    let res = srms_handler.delete(table, where_columns, where_values);
    if let Err(e) = res {
        println!("{}", e);
    } else {
        println!("删除成功！");
    }

    // 6.查找WorkPlace删除掉地址为深圳的数据后还还有哪些数据
    let select_columns = vec![];
    let where_columns = vec![];
    let where_values: Vec<Value> = vec![];
    let rows = srms_handler.select("WorkPlace", select_columns, where_columns, where_values);
    for row in rows {
        // 遍历每个字段
        for i in 0..row.len() {
            // 打印字段名和值
            println!(
                "{}: {}",
                row.columns()[i].name_str(),
                row.get::<String, _>(i).unwrap()
            );
        }
        println!("---------------------");
    }
}
