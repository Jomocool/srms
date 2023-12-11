use lazy_static::lazy_static;
use reqwest;
use serde_json::{self, json, Value};
use std::{io, sync::Mutex};

lazy_static! {
    static ref USER_NAME: Mutex<String> = Mutex::new(String::new());
    static ref USER_LEVEL: Mutex<UserLevel> = Mutex::new(UserLevel::Default);
}

#[derive(Clone, PartialEq)]
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
            println!("\n>>> 1.登录  2.注册");
            io::stdin().read_line(&mut choice).expect("无法获取选择");
            let choice = choice.trim();
            if choice.is_empty() || (choice != "1" && choice != "2") {
                println!(">>>无效的选择，请重新输入! ");
                continue;
            }
            return choice.to_string();
        }
    }
    pub fn input_username() -> String {
        loop {
            println!("\n>>> 请输入用户名(不允许有特殊符号):");
            let mut user_name = String::new();
            io::stdin()
                .read_line(&mut user_name)
                .expect("无法获取用户名");
            let user_name = user_name.trim();
            if user_name.is_empty() {
                println!(">>> 用户名不允许为空，请重新输入！");
                continue;
            }
            return user_name.to_string();
        }
    }

    pub fn input_password() -> String {
        loop {
            println!("\n>>> 请输入密码:");
            let mut password = String::new();
            io::stdin().read_line(&mut password).expect("无法获取密码");
            let password = password.trim();
            if password.is_empty() {
                println!(">>> 密码不允许为空，请重新输入！");
                continue;
            }
            return password.to_string();
        }
    }

    pub fn input_userlevel() -> String {
        loop {
            println!("\n>>> 请输入用户权限: <1.老板> <2.管理员> <3.普通员工>");
            let mut user_level = String::new();
            io::stdin()
                .read_line(&mut user_level)
                .expect("无法获取用户权限");
            let user_level = user_level.trim();
            if user_level.is_empty()
                || (user_level != "1" && user_level != "2" && user_level != "3")
            {
                println!(">>> 无效的用户权限，请重新输入！");
                continue;
            }
            return user_level.to_string();
        }
    }

    pub fn input_action() -> String {
        loop {
            println!("\n>>> 请输入你的操作:");
            println!("<0.退出>");
            println!("<1.查询>");
            println!("<2.添加>");
            println!("<3.更新>");
            println!("<4.删除>");
            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("无法获取操作");
            let action = action.trim();
            if action.is_empty()
                || (action != "0"
                    && action != "1"
                    && action != "2"
                    && action != "3"
                    && action != "4")
            {
                println!(">>> 无效的操作，请重新输入！");
                continue;
            }
            return action.to_string();
        }
    }

    pub fn input_table_name() -> String {
        loop {
            println!("\n>>> 请选择表名:");
            let mut choice = String::new();
            println!("1.办公场地表");
            println!("2.研究室表");
            println!("3.科研人员表");
            println!("4.研究室主任表");
            println!("5.秘书表");
            println!("6.科研项目表");
            println!("7.项目委托方表");
            println!("8.项目合作方表");
            println!("9.质量检测方表");
            println!("10.科研项目参与人员表");
            println!("11.科研项目子课题表");
            println!("12.科研成果表");
            println!("13.科研成果贡献人表");
            println!("14.专利表");
            println!("15.论文表");
            println!("16.软件著作权表");
            io::stdin().read_line(&mut choice).expect("无法获取选择");

            let choice = choice.trim();
            if choice.is_empty()
                || (choice != "1"
                    && choice != "2"
                    && choice != "3"
                    && choice != "4"
                    && choice != "5"
                    && choice != "6"
                    && choice != "7"
                    && choice != "8"
                    && choice != "9"
                    && choice != "10"
                    && choice != "11"
                    && choice != "12"
                    && choice != "13"
                    && choice != "14"
                    && choice != "15"
                    && choice != "16")
            {
                println!("无效的选择，请重新输入！");
                continue;
            }

            let table_name = match choice {
                "1" => "WorkPlace",
                "2" => "Lab",
                "3" => "Researcher",
                "4" => "Director",
                "5" => "Secretary",
                "6" => "ResearchProject",
                "7" => "ProjectClient",
                "8" => "ProjectCollaborator",
                "9" => "QualityMonitor",
                "10" => "ProjectParticipant",
                "11" => "ProjectSubtask",
                "12" => "ResearchOutcome",
                "13" => "OutcomeContributor",
                "14" => "Patent",
                "15" => "Paper",
                "16" => "SoftwareCopyright",
                _ => todo!(),
            };

            return table_name.to_string();
        }
    }

    pub fn input_columns() -> String {
        let mut columns_clause = String::new();
        loop {
            println!("\n>>> 请输入列名(查询所有列请输入 * ): [如果没有更多列需要被查询, 请输入0]");
            let mut column = String::new();
            io::stdin().read_line(&mut column).expect("无法获取列名");
            let column = column.trim();
            let tmp_column = column;
            if column.is_empty() {
                println!(">>> 无效的列名，请重新输入！");
                continue;
            }
            if column == "0" {
                break;
            }
            let column = format!(",{}", column);
            columns_clause.push_str(&column);
            if tmp_column == "*" {
                break;
            }
        }
        return columns_clause[1..].to_string();
    }

    pub fn input_where_clause() -> String {
        let mut where_clause = String::from("WHERE ");
        println!("\n>>> 无条件则输入0以跳过, [按下回车以输入条件]");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("无法获取选择");
        let choice = choice.trim();
        match choice {
            "0" => return "".to_string(),
            _ => loop {
                println!("\n>>> 请输入条件:");
                println!(">>> 请输入条件列名");
                let mut condition_column = String::new();
                io::stdin()
                    .read_line(&mut condition_column)
                    .expect("无法获取条件列名");
                let condition_column = condition_column.trim();
                where_clause.push_str(condition_column);

                println!(">>> 请输入条件值:");
                let mut val = String::new();
                io::stdin().read_line(&mut val).expect("无法获取条件列值");
                let val = val.trim();

                if Self::is_char_type_column(condition_column) {
                    let val = format!("='{}'", val);
                    where_clause.push_str(&val);
                } else {
                    let val = format!("={}", val);
                    where_clause.push_str(&val);
                }

                println!(
                    ">>> 请输入与下一个条件连接的逻辑关系: AND、OR... [如果没有更多条件, 请输入0]"
                );
                let mut logic_keyword = String::new();
                io::stdin()
                    .read_line(&mut logic_keyword)
                    .expect("无法获取连接词");
                let logic_keyword = logic_keyword.trim();
                if logic_keyword == "0" {
                    break;
                }
                let logic_keyword = format!(" {} ", logic_keyword);
                where_clause.push_str(&logic_keyword);
            },
        }
        return where_clause;
    }

    pub fn input_values() -> String {
        let mut values = String::new();
        loop {
            println!("\n>>> 请输入列名: [如果没有更多列, 请输入0]");
            let mut column = String::new();
            io::stdin().read_line(&mut column).expect("无法获取列名");
            let column = column.trim();
            if column.is_empty() {
                println!("无效的列名，请重新输入！");
                continue;
            }

            if column == "0" {
                break;
            }

            println!(">>> 请输入值:");
            let mut val = String::new();
            io::stdin().read_line(&mut val).expect("无法获取值");
            let val = val.trim();
            if Self::is_char_type_column(column) {
                let val = format!("'{}',", val);
                values.push_str(&val);
            } else {
                let val = format!("{},", val);
                values.push_str(&val);
            }
        }
        values.pop();
        return values;
    }

    pub fn input_set_clause() -> String {
        let mut set_clause = String::new();
        loop {
            println!("\n>>> 请输入需要更新的列名: [如果没有更多列, 请输入0]");
            let mut column = String::new();
            io::stdin().read_line(&mut column).expect("无法获取列名");
            let column = column.trim();
            if column.is_empty() {
                println!(">>> 无效的列名，请重新输入！");
                continue;
            }
            if column == "0" {
                break;
            }
            println!(">>> 请输入更新后的值:");
            let mut val = String::new();
            io::stdin().read_line(&mut val).expect("无法获取值");
            let val = val.trim();
            if Self::is_char_type_column(column) {
                let set = format!("{} = '{}',", column, val);
                set_clause.push_str(&set);
            } else {
                let set = format!("{} = {},", column, val);
                set_clause.push_str(&set);
            }
        }
        set_clause.pop();
        return set_clause;
    }

    fn is_char_type_column(column: &str) -> bool {
        return column == "address"
            || column == "name"
            || column == "introduction"
            || column == "gender"
            || column == "title"
            || column == "ResearchDirection"
            || column == "duty"
            || column == "content"
            || column == "LeaderName"
            || column == "LeaderTelephone"
            || column == "LeaderPhone"
            || column == "LeaderEmail"
            || column == "ContactName"
            || column == "ContactTelephone"
            || column == "ContactPhone"
            || column == "ContactEmail"
            || column == "TechnicalIndicators"
            || column == "type"
            || column == "StartDate"
            || column == "EndDate"
            || column == "JoinDate"
            || column == "AchieveDate";
    }
}

struct UserActionHandler {}

impl UserActionHandler {
    /// 注册
    pub fn signup() -> Value {
        let user_name = StdinHandler::input_username();
        let password = StdinHandler::input_password();
        let user_level_choice = StdinHandler::input_userlevel();
        let user_level = UserLevel::to_user_level(&user_level_choice);
        let user_level_str = UserLevel::to_string(user_level);

        let request_data = json!({
            "message_type" : "SignUp",
            "user_name" : user_name,
            "password" : password,
            "user_level" : user_level_str,
        });

        return request_data;
    }

    /// 登录
    pub fn signin() -> Value {
        let user_name = StdinHandler::input_username();
        let password = StdinHandler::input_password();
        let user_level_choice = StdinHandler::input_userlevel();
        let user_level = UserLevel::to_user_level(&user_level_choice);
        let user_level_str = UserLevel::to_string(user_level.clone());

        // 设置当前用户
        set_user(user_name.clone(), user_level);

        let request_data = json!({
            "message_type" : "SignIn",
            "user_name" : user_name,
            "password" : password,
            "user_level" : user_level_str,
        });

        return request_data;
    }

    /// 退出
    pub fn exit() -> Value {
        return json!({
            "message_type" : "Exit",
        });
    }

    pub fn select() -> Value {
        let table_name = StdinHandler::input_table_name();
        let columns = StdinHandler::input_columns();
        let where_clause = StdinHandler::input_where_clause();

        return json!({
            "message_type" : "Select",
            "table_name":table_name,
            "columns":columns,
            "where_clause":where_clause,
        });
    }

    pub fn insert() -> Value {
        let table_name = StdinHandler::input_table_name();
        let values = StdinHandler::input_values();

        return json!({
            "message_type" : "Insert",
            "table_name":table_name,
            "values":values,
        });
    }

    pub fn update() -> Value {
        let table_name = StdinHandler::input_table_name();
        let set_clause = StdinHandler::input_set_clause();
        let where_clause = StdinHandler::input_where_clause();

        return json!({
            "message_type" : "Update",
            "table_name":table_name,
            "set_clause":set_clause,
            "where_clause":where_clause,
        });
    }

    pub fn delete() -> Value {
        let table_name = StdinHandler::input_table_name();
        let where_clause = StdinHandler::input_where_clause();

        return json!({
            "message_type" : "Delete",
            "table_name":table_name,
            "where_clause":where_clause,
        });
    }
}

// 设置当前用户信息
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
            let request_data = UserActionHandler::signin();
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
            let request_data = UserActionHandler::signup();
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

    let mut exit = false;
    // 用户操作数据库
    loop {
        let action = StdinHandler::input_action();
        let request_data = match action.as_str() {
            "0" => {
                exit = true;
                UserActionHandler::exit()
            }
            "1" => UserActionHandler::select(),
            "2" => {
                if *USER_LEVEL.lock().unwrap() == UserLevel::Worker {
                    println!("\n>>> 权限不足! 无法执行操作");
                    continue;
                } else {
                    UserActionHandler::insert()
                }
            }
            "3" => {
                if *USER_LEVEL.lock().unwrap() == UserLevel::Worker {
                    println!("\n>>> 权限不足! 无法执行操作");
                    continue;
                } else {
                    UserActionHandler::update()
                }
            }
            "4" => {
                if *USER_LEVEL.lock().unwrap() == UserLevel::Worker {
                    println!("\n>>> 权限不足! 无法执行操作");
                    continue;
                } else {
                    UserActionHandler::delete()
                }
            }
            _ => todo!(),
        };
        let response = reqwest::Client::new()
            .post("http://127.0.0.1:8000")
            .json(&request_data)
            .send()
            .await
            .expect("Failed to send request!");
        let response_str = response.text().await.unwrap();
        println!("\n{}", response_str);

        if exit {
            break;
        }

        println!("\n>>> 按下任意键继续...");
        let mut enter = String::new();
        io::stdin().read_line(&mut enter).expect("无法获取按键");
    }
}
