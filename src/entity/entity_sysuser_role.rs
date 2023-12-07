use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetUserRole {
    pub id:i64,
    pub userid:i64,
    pub roleid:i64
}

rbatis::crud!(NetUserRole{},"net_userrole");