use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;



/// 绑定主机,端口
#[derive(Deserialize, Default, Debug)]
pub struct App {
    pub host: String,
    pub port: u16,
}

/// 数据库连接信息
#[derive(Deserialize, Default, Debug)]
pub struct Database {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: usize,
    pub param: String,
}

/// 日志信息
#[derive(Deserialize, Default, Debug)]
pub struct Log {
    pub level: String,
    pub path: String,
}
/// 系统配置信息
#[derive(Deserialize, Default, Debug)]
pub struct Setting {
    pub app: App,
    pub database: Database,
    pub log: Log,
}


/// 获取toml相关配置
macro_rules! get_setting_from_toml {
    ($struct: ident) => {{
        let result = $struct::default();

        // 获取项目的目录路径
        // let current_dir = if let Ok(v) = env::current_dir() { v } else { return result; };
        // let current_path = if let Some(v) = current_dir.to_str() { v } else { return result; };
        let current_path = env!("CARGO_MANIFEST_DIR");
        // 读取配置文件
        let toml_file = format!("{}/configs/config.toml", current_path);
        match File::open(&toml_file) {
            Ok(mut v) => {
                let mut content = String::new();
                if let Ok(_) = v.read_to_string(&mut content) {
                    if let Ok(t) = toml::from_str::<$struct>(&content) {
                        t
                    } else {
                        result
                    }
                } else {
                    result
                }
            }
            Err(err) => {
                println!("读取文件失败: {}", err);
                result
            }
        }
    }};
}

lazy_static! {
    pub static ref SETTING: Setting = get_setting_from_toml!(Setting);
}

/// 得到数据库连接字符串
pub fn get_conn_string() -> String {
    let setting = &*SETTING;
    let db = &setting.database;
    format!(
        "mysql://{}:{}@{}:{}/{}?{}",
        db.user, db.password, db.host, db.port, db.name,db.param
    )
}

/// 得到日志级别
#[allow(dead_code)]
pub fn get_log_level() -> String {
    let setting = &*SETTING;
    let log = &setting.log;
    format!("{}", log.level)
}

pub fn get_log_path() -> String{
    let setting = &*SETTING;
    let log = &setting.log;
    format!("{}", log.path)
}


#[cfg(test)]
mod config_tests {

    use super::*;

    #[tokio::test]
    async fn test_get_setting() {
        let setting = &*SETTING;
        println!("{:?}", setting);
    }

    #[tokio::test]
    async fn test_get_conn_string() {
        println!("{:?}", get_conn_string());
    }
}
