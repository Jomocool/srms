use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

use chrono::Local;
use hyper::body::Bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::{self, Value as JsonValue};

use crate::database::{DBError, DBHandler};

mod database;

lazy_static! {
    static ref SRMS_HANDLER: Mutex<DBHandler> = Mutex::new(DBHandler::new());
    static ref USER_MANAGER: Mutex<UserManager> = Mutex::new(UserManager::new());
    static ref USER: Mutex<User> = Mutex::new(User::new());
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Select {
    message_type: String,
    table_name: String,
    columns: String,
    where_clause: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Insert {
    message_type: String,
    table_name: String,
    values: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Update {
    message_type: String,
    table_name: String,
    set_clause: String,
    where_clause: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Delete {
    message_type: String,
    table_name: String,
    where_clause: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Exit {
    message_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct User {
    message_type: String,
    user_name: String,
    password: String,
    user_level: String,
}

impl User {
    pub fn new() -> User {
        User {
            message_type: String::new(),
            user_name: String::new(),
            password: String::new(),
            user_level: String::new(),
        }
    }
}

struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager { users: Vec::new() }
    }

    pub fn add_user(&mut self, user: &User) -> bool {
        if self.is_user_exist(&user.user_name).is_some() {
            return false;
        }
        self.users.push(user.clone());
        return true;
    }

    /// 判断用户名是否已存在
    pub fn is_user_exist(&self, user_name: &str) -> Option<User> {
        for user in self.users.iter() {
            if user.user_name == user_name {
                return Some(user.clone());
            }
        }
        return None;
    }

    /// 判断用户信息是否正确（用户存在且密码正确）
    pub fn is_match(&self, user: &User) -> bool {
        if let Some(r) = self.is_user_exist(&user.user_name) {
            return r.password == user.password && r.user_level == user.user_level;
        }
        return false;
    }
}

struct ServerHandler {}

impl ServerHandler {
    pub fn handle_user_signin(body_bytes: Bytes) -> String {
        let user: User = serde_json::from_slice(&body_bytes).unwrap();
        if USER_MANAGER.lock().unwrap().is_match(&user) {
            *USER.lock().unwrap() = user.clone();

            // 打日志
            let action = format!(
                "{}({})   {}",
                user.user_name, user.user_level, user.message_type
            );
            Self::log(action);

            let success_log = format!(
                "登录成功，欢迎使用科研管理系统！\n你好! {}",
                &user.user_name
            );
            return success_log;
        }
        return "登录失败，无效的用户名或密码、权限！".to_string();
    }

    pub fn handle_user_signup(body_bytes: Bytes) -> String {
        let user: User = serde_json::from_slice(&body_bytes).unwrap();
        if USER_MANAGER.lock().unwrap().add_user(&user) {
            // 打日志
            let action = format!(
                "{}({})   {}",
                user.user_name, user.user_level, user.message_type
            );
            Self::log(action);
            return "注册成功! 请登录~".to_string();
        }
        return "注册失败! 用户名已存在，请更换用户名".to_string();
    }

    pub fn handle_select(body_bytes: Bytes) -> String {
        let select: Select = serde_json::from_slice(&body_bytes).unwrap();

        let message = match SRMS_HANDLER.lock().unwrap().select(
            select.table_name.clone(),
            select.columns,
            select.where_clause,
        ) {
            Ok(str) => {
                // 打日志
                let action = format!(
                    "{}   {} from {}",
                    Self::user(),
                    select.message_type,
                    select.table_name
                );
                Self::log(action);

                str
            }
            Err(DBError::SelectError(e)) => e,
            _ => todo!(),
        };

        return message;
    }

    pub fn handle_insert(body_bytes: Bytes) -> String {
        let insert: Insert = serde_json::from_slice(&body_bytes).unwrap();
        let message = match SRMS_HANDLER
            .lock()
            .unwrap()
            .insert(insert.table_name.clone(), insert.values)
        {
            Ok(str) => {
                // 打日志
                let action = format!(
                    "{}   {} to {}",
                    Self::user(),
                    insert.message_type,
                    insert.table_name
                );
                Self::log(action);

                str
            }
            Err(DBError::InsertError(e)) => e,
            _ => todo!(),
        };

        return message;
    }

    pub fn hadnle_update(body_bytes: Bytes) -> String {
        let update: Update = serde_json::from_slice(&body_bytes).unwrap();

        let message = match SRMS_HANDLER.lock().unwrap().update(
            update.table_name.clone(),
            update.set_clause,
            update.where_clause,
        ) {
            Ok(str) => {
                // 打日志
                let action = format!(
                    "{}   {} {}",
                    Self::user(),
                    update.message_type,
                    update.table_name
                );
                Self::log(action);

                str
            }
            Err(DBError::UpdateError(e)) => e,
            _ => todo!(),
        };

        return message;
    }

    pub fn hadnle_delete(body_bytes: Bytes) -> String {
        let delete: Delete = serde_json::from_slice(&body_bytes).unwrap();
        let message = match SRMS_HANDLER
            .lock()
            .unwrap()
            .delete(delete.table_name.clone(), delete.where_clause)
        {
            Ok(str) => {
                // 打日志
                let action = format!(
                    "{}   {} data from {}",
                    Self::user(),
                    delete.message_type,
                    delete.table_name
                );
                Self::log(action);

                str
            }
            Err(DBError::UpdateError(e)) => e,
            _ => todo!(),
        };

        return message;
    }

    pub fn handle_exit(body_bytes: Bytes) -> String {
        let exit: Exit = serde_json::from_slice(&body_bytes).unwrap();
        // 打日志
        let action = format!("{}   {}", Self::user(), exit.message_type,);
        Self::log(action);

        return "Bye~".to_string();
    }

    fn log(action: String) {
        // 时间
        let current_time = Local::now();
        let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

        // 完整信息
        let log_data = format!("{}   {}\n", formatted_time, action);
        println!("{log_data}");

        // 写入日志文件
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("srms.log")
            .expect("Failed to open srms.log.");
        file.write_all(log_data.as_bytes()).expect("Failed to log");
    }

    fn user() -> String {
        let guard = USER.lock().unwrap();
        format!("{}({})", guard.user_name, guard.user_level)
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.method() == hyper::Method::POST {
        // 将消息转为字符串，以判断用哪个结构体去接收
        let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
        let request_data: JsonValue = serde_json::from_slice(&body_bytes).unwrap();

        // 根据请求数据的内容选择相应的结构体
        let response: String = match request_data.get("message_type") {
            Some(JsonValue::String(message_type)) => match message_type.as_str() {
                "Exit" => ServerHandler::handle_exit(body_bytes),
                "SignIn" => ServerHandler::handle_user_signin(body_bytes),
                "SignUp" => ServerHandler::handle_user_signup(body_bytes),
                "Select" => ServerHandler::handle_select(body_bytes),
                "Insert" => ServerHandler::handle_insert(body_bytes),
                "Update" => ServerHandler::hadnle_update(body_bytes),
                "Delete" => ServerHandler::hadnle_delete(body_bytes),
                _ => panic!("Unkown message type"),
            },
            _ => panic!("Missing or invalid message_type"),
        };

        let body = Body::from(response);
        return Ok(Response::new(body));
    }

    // 处理服务端请求逻辑，这里简单返回一个响应
    let body = Body::from("Hello, this is the server!");
    Ok(Response::new(body))
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8000).into();
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server is running on http://{}", addr);

    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
