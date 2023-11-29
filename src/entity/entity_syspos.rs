use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::{ RBatis, py_sql};


/// 职位
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysPos {
    
    pub id: i64,  
    pub name: Option<String>,
    pub code: Option<String>,
    pub orderno: i32,
    pub remark: Option<String>,
    pub status: i8,
   
}

/// 获取所有菜单
/// accounttype 999 超级管理员
#[py_sql("`select * from net_syspos as c  where 1=1 `
if name!='':
and c.name like '%${name}%'
if code!='':
` and c.code like '%${code}%' `
order by c.code ")]
async fn get_pos(rb: &RBatis, name:&str,code:&str) -> rbatis::Result<Vec<NetSysPos>> { impled!() }