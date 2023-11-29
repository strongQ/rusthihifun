
use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::{rbdc::datetime::DateTime, RBatis, sql, py_sql};

use crate::model::user_model::LoginUserRes;

use super::{entity_sysorg::NetSysOrg, entity_syspos::NetSysPos};



#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct NetSysUser {
    
    pub id: i64,
    pub account: String,
    pub accounttype: i32,
    pub password: Option<String>,
    pub realname:Option<String>,
    // 昵称
    pub nickname:Option<String>,
    pub avatar:Option<String>,
    // 男1 女2
    pub sex:i8,
    pub age:i32,
    pub birthday:Option<DateTime>,
    pub nation:Option<String>,
    pub phone:Option<String>,
    pub cardtype:i8,
    pub idcardnum:Option<String>,
    pub email:Option<String>,
    pub address:Option<String>,
    pub culturelevel:i8,
    pub politicaloutlook:Option<String>,
    pub college:Option<String>,
    pub officephone:Option<String>,
    pub emergencycontact:Option<String>,
    pub emergencyphone:Option<String>,
    pub emergencyaddress:Option<String>,
    pub introduction:Option<String>,
    pub orgid:i64,
    pub sysorg:Option<NetSysOrg>,
    pub posid:i64,
    pub syspos:Option<NetSysPos>,
    pub jobnum:Option<String>,
    pub poslevel:Option<String>,
    pub joindate:Option<DateTime>,
    pub lastloginip:Option<String>,
    pub lastloginaddress:Option<String>,
    pub lastlogintime:Option<DateTime>,
    pub lastlogindevice:Option<String>,
    pub signature:Option<String>,
    pub status:i8,
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
pub struct NetSysUserDto {
    pub id: i64,
    pub status:i8,
    pub account: String,
    pub accounttype: i32,
    pub password: Option<String>,
    
}
 impl Default  for NetSysUserDto {
    fn default() -> Self {
        Self { id: Default::default(), status: Default::default(), account: Default::default(), accounttype: Default::default(), password: Default::default() }
    }
}



rbatis::crud!(NetSysUser{},"net_sysuser");

/// 获取用户主要信息
#[sql("select id,account,password,status,accounttype from net_sysuser where account=?  limit 1")]
async fn get_userdto(rb: &RBatis, account: &String) -> rbatis::Result<NetSysUserDto> {
    impled!()
}
/// 获取用户相关信息
#[sql("SELECT a.account,a.realname,a.avatar,a.address,a.signature,a.orgid,b.name as orgname,c.name as postname FROM net_sysuser a left join net_syspos b on a.orgid=b.id left join net_syspos c on a.posid=c.id where a.id=?  limit 1")]
async fn get_userinfo(rb: &RBatis, id: i64) -> rbatis::Result<LoginUserRes> {
    impled!()
}

/// 用户分页
/// accounttype 999 超级管理员
#[py_sql(" with RECURSIVE cte as (select * from net_sysorg 
      if orgid>0:
        ` where id=${orgid} `

      if accounttype!=999 && orgid==0:
        ` where id=(select orgid from net_sysuser where id=${userid}) `

` union all select a.* from net_sysorg as a INNER JOIN cte on a.pid=cte.id), `
 t2 as (select DISTINCT(id) from cte union all select  orgid  from net_userextorg as a where a.userid =${userid}) select u.* from net_sysuser as u inner join t2 on u.orgid=t2.id where 1=1 

     if account!='':
       ` and u.account like '%${account}%' `
     if realname!='':
       ` and u.realname like '%${realname}%' `
     if phone!='':
       ` and u.phone like '%${phone}%' `

` order by u.orderno limit ${pagesize} OFFSET ${pagesize}*(${page}-1) ` ")]
async fn get_user_page(rb: &RBatis, account:&str,realname:&str,phone:&str,pagesize:u32,page:u32, userid:i64,accounttype:i32,orgid:i64) -> rbatis::Result<Vec<NetSysUser>> { impled!() }

/// 用户分页
/// accounttype 999 超级管理员
#[py_sql(" with RECURSIVE cte as (select * from net_sysorg 
    if orgid>0:
      ` where id=${orgid} `

    if accounttype!=999 && orgid==0:
      ` where id=(select orgid from net_sysuser where id=${userid}) `

` union all select a.* from net_sysorg as a INNER JOIN cte on a.pid=cte.id), `
t2 as (select DISTINCT(id) from cte union all select  orgid  from net_userextorg as a where a.userid =${userid}) select count(1) from net_sysuser as u inner join t2 on u.orgid=t2.id where 1=1 

   if account!='':
     ` and u.account like '%${account}%' `
   if realname!='':
     ` and u.realname like '%${realname}%' `
   if phone!='':
     ` and u.phone like '%${phone}%' `



")]
async fn get_user_total(rb: &RBatis, account:&str,realname:&str,phone:&str, userid:i64,accounttype:i32,orgid:i64) -> rbatis::Result<u64> { impled!() }
