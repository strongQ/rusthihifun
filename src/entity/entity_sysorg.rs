

use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::{ RBatis, py_sql};


/// 组织
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysOrg {
    
    pub id: i64,
    pub pid:i64,
    pub name: Option<String>,
    pub code: Option<String>,
    pub orderno: i32,
    pub remark: Option<String>,
    pub status: i8,
    pub  children:Option<Vec<NetSysOrg>>
}

/// 获取所有菜单
/// accounttype 999 超级管理员
#[py_sql(" `with RECURSIVE cte as(select * from net_sysorg ` 

        if accounttype != 999:
           ` where id=(select orgid from net_sysuser where id=${userid}) `

 union all select a.* from net_sysorg as a INNER JOIN cte on a.pid=cte.id),
 t2 as
(select DISTINCT(id) from cte union select  orgid  from net_userextorg as a ) select c.* from t2 inner join net_sysorg as c on t2.id=c.id where 1=1 

       if name!='':
         ` and c.name like '%${name}%' `
       if code!='':
         ` and c.code like '%${code}%' `

` order by c.code` ")]
async fn get_user_orgs(rb: &RBatis,userid:i64,accounttype:i32,name:&str,code:&str) -> rbatis::Result<Vec<NetSysOrg>> { impled!() }
