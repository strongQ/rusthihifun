

use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::{rbdc::datetime::DateTime, sql, RBatis};


#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysConfig {
    
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub value: Option<String>,
    pub sysflag: i32,
    pub groupcode: Option<String>,
    pub orderno: i32,
    pub remark: Option<String>,
    pub createtime: Option<DateTime>,
    pub updatetime: Option<DateTime>,
    pub createuserid: Option<i64>,
    pub updateuserid: Option<i64>,
    pub isdelete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysConfigDto {
    
    
    pub code: Option<String>,
    pub value: Option<String>,
    
}

/// doc you can see https://rbatis.github.io/rbatis.io
#[sql("select value from net_sysconfig where code=?  limit 1")]
async fn get_value(rb: &RBatis, code: &String) -> rbatis::Result<String> {
    impled!()
}


rbatis::crud!(NetSysConfig{},"net_sysconfig");




