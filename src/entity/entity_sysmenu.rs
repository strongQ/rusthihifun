

use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::{rbdc::datetime::DateTime, RBatis, py_sql};

use crate::model::menu_model::SysMenuRes;


#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysMenu {
    
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
    pub createtime: Option<DateTime>,
    pub updatetime: Option<DateTime>,
    pub createuserid: Option<i64>,
    pub updateuserid: Option<i64>,
    pub isdelete: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysMenuDto{
    pub permission: Option<String>
}

rbatis::crud!(NetSysMenu{},"net_sysmenu");



/// 获取菜单权限 
/// type 菜单类型（1目录 2菜单 3按钮）
#[py_sql(" `with t as( select distinct(b.menuid) from net_userrole as a left join net_sysrolemenu as b on a.roleid=b.roleid    where a.userid=#{userid})
select distinct(c.permission) from net_sysmenu as c  `
if accounttype==999:
 left join
if accounttype!=999:
 inner join

 ` t on c.id=t.menuid  where c.permission is not null and c.TYPE=#{r#type}`")]
async fn get_menu_permissions(rb: &RBatis, userid:i64,r#type:i8,accounttype:i32) -> rbatis::Result<Vec<NetSysMenuDto>> { impled!() }



/// 获取所有菜单
/// accounttype 999 超级管理员
#[py_sql(" `with t as( select distinct(b.menuid) from net_userrole as a left join net_sysrolemenu as b on a.roleid=b.roleid    where a.userid=#{userid})
select * from net_sysmenu as a  `
if accounttype==999:
 left join
if accounttype!=999:
 inner join

 ` t on a.id=t.menuid where a.type<>3 and a.status=1 order by orderno`")]
async fn get_menus(rb: &RBatis, userid:i64,accounttype:i32) -> rbatis::Result<Vec<SysMenuRes>> { impled!() }




