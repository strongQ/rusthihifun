use rbatis::rbdc::DateTime;
use salvo::oapi::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysNotice {
    pub id: i64,
    pub title:String,
    pub content: String,
    // 类型（1通知 2公告）
    pub r#type: i8  ,
    pub publicuserid:i64,
    pub publicusername:Option<String>,
    pub publicorgid:i64,
    pub publicorgname:Option<String>,
    pub publictime:Option<DateTime>,
    pub canceltime:Option<DateTime>,
    // 状态（0草稿 1发布 2撤回 3删除）
    pub status:i8
}