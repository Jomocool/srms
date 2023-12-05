use std::sync::Mutex;

use hyper::body::Bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::{self, Value as JsonValue};

use crate::database::DBHandler;

mod database;

lazy_static! {
    static ref SRMS_HANDLER: Mutex<DBHandler> = Mutex::new(DBHandler::new());
    static ref USER_MANAGER: Mutex<UserManager> = Mutex::new(UserManager::new());
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct User {
    message_type: String,
    user_name: String,
    password: String,
    user_level: String,
}

struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager { users: Vec::new() }
    }

    pub fn add_user(&mut self, user: User) -> bool {
        if self.is_user_exist(user.user_name.clone()).is_some() {
            return false;
        }
        self.users.push(user);
        return true;
    }

    /// 判断用户名是否已存在
    pub fn is_user_exist(&self, user_name: String) -> Option<User> {
        for user in self.users.iter() {
            if user.user_name == user_name {
                return Some(user.clone());
            }
        }
        return None;
    }

    /// 判断用户信息是否正确（用户存在且密码正确）
    pub fn is_match(&self, user: User) -> bool {
        if let Some(r) = self.is_user_exist(user.user_name) {
            return r.password == user.password && r.user_level == user.user_level;
        }
        return false;
    }
}

fn handle_user_signin(body_bytes: Bytes) -> String {
    let user: User = serde_json::from_slice(&body_bytes).unwrap();
    if USER_MANAGER.lock().unwrap().is_match(user) {
        return "登录成功，欢迎使用科研管理系统！".to_string();
    }
    return "登录失败，无效的用户名或密码、权限！".to_string();
}

fn handle_user_signup(body_bytes: Bytes) -> String {
    let user: User = serde_json::from_slice(&body_bytes).unwrap();
    if USER_MANAGER.lock().unwrap().add_user(user) {
        return "注册成功! 请登录~".to_string();
    }
    return "注册失败! 用户名已存在，请更换用户名".to_string();
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.method() == hyper::Method::POST {
        // 将消息转为字符串，以判断用哪个结构体去接收
        let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
        let request_data: JsonValue = serde_json::from_slice(&body_bytes).unwrap();

        // 根据请求数据的内容选择相应的结构体
        let response: String = match request_data.get("message_type") {
            Some(JsonValue::String(message_type)) => match message_type.as_str() {
                "UserSignIn" => handle_user_signin(body_bytes),
                "UserSignUp" => handle_user_signup(body_bytes),
                _ => panic!("Unkown message type"),
            },
            _ => panic!("Missing or invalid message_type"),
        };

        let body = Body::from(response);
        return Ok(Response::new(body));

        // let mut response_body = String::new();
        // match request_data.message.as_str() {
        //     "1" => {
        //         response_body.push_str("------------------------------------------\n");
        //         let select_columns = vec![];
        //         let where_columns = vec![];
        //         let where_values: Vec<Value> = vec![];
        //         let rows = SRMS_HANDLER.lock().unwrap().select(
        //             "WorkPlace",
        //             select_columns,
        //             where_columns,
        //             where_values,
        //         );
        //         for row in rows {
        //             // 遍历每个字段
        //             let mut row_str = String::new();
        //             for i in 0..row.len() {
        //                 let col_val = format!(
        //                     "| {}: {} ",
        //                     &row.columns()[i].name_str(),
        //                     row.get::<String, _>(i).unwrap()
        //                 );
        //                 row_str.push_str(&col_val);
        //             }
        //             row_str.push_str("\n");
        //             response_body.push_str(&row_str);
        //             response_body.push_str("------------------------------------------\n");
        //         }
        //         response_body.push('\n');
        //     }
        //     _ => todo!(),
        // }

        // let body = Body::from(response_body);
        // return Ok(Response::new(body));
    }

    // 处理服务端请求逻辑，这里简单返回一个响应
    let body = Body::from("Hello, this is the server!");
    Ok(Response::new(body))
}

#[tokio::main]
async fn main() {
    let srms_handler = DBHandler::new();

    let addr = ([127, 0, 0, 1], 8000).into();
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server is running on http://{}", addr);

    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}

// use database::DBHandler;
// use mysql::Value;

// pub mod database;

// fn main() {
//     let mut srms_handler = DBHandler::new();
//     let table = "WorkPlace";

//     // 1. 插入数据到WorkPlace
//     let columns = vec!["id", "area", "address"];
//     let values = vec![Value::from(1), Value::from(100), Value::from("中国")];
//     let res = srms_handler.insert(table, columns, values);
//     if let Err(e) = res {
//         println!("{}", e);
//     } else {
//         println!("插入成功！");
//     }

//     // 2. 查找WorkPlace所有数据
//     let select_columns = vec![];
//     let where_columns = vec![];
//     let where_values: Vec<Value> = vec![];
//     let rows = srms_handler.select("WorkPlace", select_columns, where_columns, where_values);
//     for row in rows {
//         // 遍历每个字段
//         for i in 0..row.len() {
//             // 打印字段名和值
//             println!(
//                 "{}: {}",
//                 row.columns()[i].name_str(),
//                 row.get::<String, _>(i).unwrap()
//             );
//         }
//         println!("---------------------");
//     }

//     // 3. 更新WorkPlace表中所有地区为中国的数据，将地址改为深圳
//     let set_columns = vec!["address"];
//     let set_values = vec![Value::from("深圳")];
//     let where_columns = vec!["address"];
//     let where_values = vec![Value::from("中国")];
//     let res = srms_handler.update(table, set_columns, set_values, where_columns, where_values);
//     if let Err(e) = res {
//         println!("{}", e);
//     } else {
//         println!("更新成功！");
//     }

//     // 4. 查找WorkPlace更新后的数据
//     let select_columns = vec![];
//     let where_columns = vec![];
//     let where_values: Vec<Value> = vec![];
//     let rows = srms_handler.select("WorkPlace", select_columns, where_columns, where_values);
//     for row in rows {
//         // 遍历每个字段
//         for i in 0..row.len() {
//             // 打印字段名和值
//             println!(
//                 "{}: {}",
//                 row.columns()[i].name_str(),
//                 row.get::<String, _>(i).unwrap()
//             );
//         }
//         println!("---------------------");
//     }

//     // 5. 删掉WorkPlace中地址为深圳的数据
//     let where_columns = vec!["address"];
//     let where_values = vec![Value::from("深圳")];
//     let res = srms_handler.delete(table, where_columns, where_values);
//     if let Err(e) = res {
//         println!("{}", e);
//     } else {
//         println!("删除成功！");
//     }

//     // 6.查找WorkPlace删除掉地址为深圳的数据后还还有哪些数据
//     let select_columns = vec![];
//     let where_columns = vec![];
//     let where_values: Vec<Value> = vec![];
//     let rows = srms_handler.select("WorkPlace", select_columns, where_columns, where_values);
//     for row in rows {
//         // 遍历每个字段
//         for i in 0..row.len() {
//             // 打印字段名和值
//             println!(
//                 "{}: {}",
//                 row.columns()[i].name_str(),
//                 row.get::<String, _>(i).unwrap()
//             );
//         }
//         println!("---------------------");
//     }
// }
