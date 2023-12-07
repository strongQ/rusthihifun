use rbatis::{RBatis, sql, py_sql};
use salvo::oapi::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleOutput {
    pub id: i64,
    pub name:String,
    pub code: Option<String> 
}

#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleIdOutput {
    pub roleid: i64  
}

/// 获取所有角色
#[sql("select id,code,name from net_sysrole order by orderno")]
async fn get_role(rb: &RBatis) -> rbatis::Result<Vec<SysRoleOutput>> { impled!() }

/// 根据用户Id获取角色Id集合
#[py_sql("select distinct roleid from net_userrole where userid=${userid}")]
async fn get_own_rolelist(rb: &RBatis,userid:i64) -> rbatis::Result<Vec<SysRoleIdOutput>> { impled!() }