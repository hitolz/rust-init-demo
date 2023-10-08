use std::{ops::DerefMut, sync::Mutex};

use crate::error::Result;
use actix_web::web;
use lazy_static::*;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub type Db = web::Data<Mutex<Pool<MySql>>>;

lazy_static! {
    static ref POOLS: Mutex<Vec<Pool<MySql>>> = Mutex::new(vec![]);
}

lazy_static! {
    static ref RB: Mutex<RBatis> = Mutex::new(RBatis::new());
}

#[macro_export]
macro_rules! pool {
    ($pool: expr) => {{
        crate::db::get($pool)
    }};
}

#[macro_export]
macro_rules! from_row {
    ($row: expr) => {{
        mysql::from_row($row)
    }};
}

/// Initialize mysql connection pool
pub async fn init_connections(conn_string: &str) -> Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .idle_timeout(Some(std::time::Duration::from_secs(30)))
        .connect(conn_string)
        .await?;
    let mut pools = POOLS.lock().unwrap();
    (*pools).push(pool);

    let rbatis = RBatis::new();
    rbatis.init(
        MysqlDriver {},
        conn_string,
    )
        .unwrap();

    let mut rb = RB.lock().unwrap();
    *rb = rbatis;
    Ok(())
}

/// get pool
pub fn get_pool() -> Pool<MySql> {
    let pools = POOLS.lock().unwrap();
    unsafe { (*pools).get_unchecked(0).to_owned() }
}

/// deref the reference of pool
pub fn get(pool: &Db) -> Pool<MySql> {
    pool.lock().unwrap().deref_mut().to_owned()
}

pub fn get_rb() -> RBatis{
    let x = RB.lock().unwrap().clone();
    x
}

#[cfg(test)]
mod db_tests {
    use sqlx::Row;

    use super::*;

    #[tokio::test]
    async fn test_get_pool() {
        let url = "mysql://root:12345678@127.0.0.1:3306/test";
        let _ = init_connections(url).await;
        let pool = get_pool();
        let sql = "select * from t1 where id < ?";
        let mut rows = sqlx::query(sql).bind(10).fetch_all(&pool).await.unwrap();
        while let Some(row) = rows.pop() {
            let id: u64 = row.try_get("id").unwrap();
            println!("id = {}", id);
        }
    }
}
