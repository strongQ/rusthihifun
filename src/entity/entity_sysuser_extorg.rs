use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::{ RBatis, py_sql, rbdc::DateTime};
/// 系统用户扩展机构表
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetUserExtOrg {
    
    pub id: i64,
    pub userid:i64,
    pub orgid:i64,
    pub posid:i64,
    pub jobnum: Option<String>,
    pub poslevel: Option<String>,
    pub joindate: Option<DateTime>,
}

rbatis::crud!(NetUserExtOrg{},"net_userextorg");

/// 获取用户扩展机构集合
#[py_sql("select * from net_userextorg nu  where userid=${userid}")]
async fn get_ext_orgs(rb: &RBatis,userid:i64) -> rbatis::Result<Vec<NetUserExtOrg>> { impled!() }