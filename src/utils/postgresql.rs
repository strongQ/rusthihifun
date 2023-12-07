use crate::GLOBAL_DB;
/// 连接数据库
pub async fn init_db(){
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    GLOBAL_DB.init(rbdc_pg::driver::PgDriver{},"postgres://postgres:pwd@ip:port/db").unwrap();
}