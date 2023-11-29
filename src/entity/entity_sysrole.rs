use rbatis::{RBatis, sql};
use salvo::oapi::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleOutput {
    pub id: i64,
    pub name:String,
    pub code: Option<String> 
}

/// 获取所有角色
#[sql("select id,code,name from net_sysrole order by orderno")]
async fn get_role(rb: &RBatis) -> rbatis::Result<Vec<SysRoleOutput>> { impled!() }