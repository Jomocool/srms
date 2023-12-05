use lazy_static::lazy_static;
use reqwest;
use serde_json::{self, json, Value};
use std::{io, sync::Mutex};

lazy_static! {
    static ref USER_NAME: Mutex<String> = Mutex::new(String::new());
    static ref USER_LEVEL: Mutex<UserLevel> = Mutex::new(UserLevel::Default);
}

#[derive(Clone)]
/// 用户级别
enum UserLevel {
    /// 老板
    Boss,
    /// 管理员
    Manager,
    /// 普通员工
    Worker,
    /// 默认
    Default,
}

impl UserLevel {
    pub fn to_user_level(choice: &str) -> UserLevel {
        match choice {
            "1" => UserLevel::Boss,
            "2" => UserLevel::Manager,
            "3" => UserLevel::Worker,
            _ => todo!(),
        }
    }

    pub fn to_string(user_level: UserLevel) -> String {
        match user_level {
            UserLevel::Boss => "Boss".to_string(),
            UserLevel::Manager => "Manager".to_string(),
            UserLevel::Worker => "Worker".to_string(),
            _ => String::new(),
        }
    }
}

struct StdinHandler {}

impl StdinHandler {
    pub fn input_choice() -> String {
        loop {
            let mut choice = String::new();
            println!("\n<1.登录>  <2.注册>");
            io::stdin().read_line(&mut choice).expect("无法获取选择");
            let choice = choice.trim();
            if choice.is_empty() || (choice != "1" && choice != "2") {
                println!("无效的选择，请重新输入! ");
                continue;
            }
            return choice.to_string();
        }
    }
    pub fn input_username() -> String {
        loop {
            println!("请输入用户名(不允许有特殊符号):");
            let mut user_name = String::new();
            io::stdin()
                .read_line(&mut user_name)
                .expect("无法获取用户名");
            let user_name = user_name.trim();
            if user_name.is_empty() {
                println!("用户名不允许为空，请重新输入！");
                continue;
            }
            return user_name.to_string();
        }
    }

    pub fn input_password() -> String {
        loop {
            println!("请输入密码:");
            let mut password = String::new();
            io::stdin().read_line(&mut password).expect("无法获取密码");
            let password = password.trim();
            if password.is_empty() {
                println!("密码不允许为空，请重新输入！");
                continue;
            }
            return password.to_string();
        }
    }

    pub fn input_userlevel() -> String {
        loop {
            println!("请输入用户权限: <1.老板> <2.管理员> <3.普通员工>");
            let mut user_level = String::new();
            io::stdin()
                .read_line(&mut user_level)
                .expect("无法获取用户权限");
            let user_level = user_level.trim();
            if user_level.is_empty()
                || (user_level != "1" && user_level != "2" && user_level != "3")
            {
                println!("无效的用户权限，请重新输入！");
                continue;
            }
            return user_level.to_string();
        }
    }
}

/// 注册
fn signup() -> Value {
    let user_name = StdinHandler::input_username();
    let password = StdinHandler::input_password();
    let user_level_choice = StdinHandler::input_userlevel();
    let user_level = UserLevel::to_user_level(&user_level_choice);
    let user_level_str = UserLevel::to_string(user_level);

    let request_data = json!({
        "message_type" : "UserSignUp",
        "user_name" : user_name,
        "password" : password,
        "user_level" : user_level_str,
    });

    return request_data;
}

/// 登录
fn signin() -> Value {
    let user_name = StdinHandler::input_username();
    let password = StdinHandler::input_password();
    let user_level_choice = StdinHandler::input_userlevel();
    let user_level = UserLevel::to_user_level(&user_level_choice);
    let user_level_str = UserLevel::to_string(user_level.clone());

    // 设置当前用户
    set_user(user_name.clone(), user_level);

    let request_data = json!({
        "message_type" : "UserSignIn",
        "user_name" : user_name,
        "password" : password,
        "user_level" : user_level_str,
    });

    return request_data;
}

fn set_user(user_name: String, user_level: UserLevel) {
    *USER_NAME.lock().unwrap() = user_name;
    *USER_LEVEL.lock().unwrap() = user_level;
}

#[tokio::main]
async fn main() {
    // 用户登录或注册
    loop {
        let choice = StdinHandler::input_choice();
        if choice == "1" {
            let request_data = signin();
            let response = reqwest::Client::new()
                .post("http://127.0.0.1:8000")
                .json(&request_data)
                .send()
                .await
                .expect("Failed to send request!");
            let response_str = response.text().await.unwrap();
            println!("{}", response_str);
            if response_str.contains("登录成功") {
                break;
            }
            continue;
        } else if choice == "2" {
            let request_data = signup();
            let response = reqwest::Client::new()
                .post("http://127.0.0.1:8000")
                .json(&request_data)
                .send()
                .await
                .expect("Failed to send request!");
            let response_str = response.text().await.unwrap();
            println!("{}", response_str);
            continue;
        }
    }
}
