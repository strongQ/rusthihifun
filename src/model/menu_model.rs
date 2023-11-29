


use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;


#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub  struct SysMenuRes {
    
    pub id: i64,
    pub pid:i64,
    //  菜单类型（0目录 1菜单 2按钮）
    pub r#type:i8,
    pub name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>,
    pub permission: Option<String>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub isiframe:bool,
    pub ishide:bool,
    pub iskeepalive:bool,
    pub outlink:Option<String>,
    pub isaffix:bool,
    pub createtime: Option<DateTime>,
    pub updatetime: Option<DateTime>,
    pub meta:Option<SysMenuMeta>,
    pub  children:Option<Vec<SysMenuRes>>
     
}
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysMenuMeta{
   pub title:Option<String>,
   pub icon:Option<String>,
   pub isiframe:bool,
   pub islink:Option<String>,
   pub ishide:bool,
   pub iskeepalive:bool,
   pub isaffix:bool
}