

use idgenerator::IdInstance;
use rbatis::Error;
use salvo::oapi::extract::PathParam;

use crate::{entity::{entity_sysrole::{SysRoleOutput, self, SysRoleIdOutput}, entity_sysuser::NetSysUser, entity_sysuser_role::{self, NetUserRole}, entity_sysuser_extorg::{self, NetUserExtOrg}}, GLOBAL_DB};



/// 获取角色列表
pub async fn get_roles()->Result<Vec<SysRoleOutput>,Error>{

   
   let db= &GLOBAL_DB.clone();
   let result=entity_sysrole::get_role(db).await;
   match result{
       Ok(pos)=>{
 
         
         return  Ok(pos);
       },
       Err(err)=>{
         return  Err(err);
       }
     }
}
/// 根据用户Id获取角色Id集合
pub async fn get_own_rolelist(id:PathParam<Option<i64>>)->Result<Vec<SysRoleIdOutput>,Error>{

  let userid=id.into_inner().unwrap();
  let db= &GLOBAL_DB.clone();
   let result=entity_sysrole::get_own_rolelist(db,userid).await;
   result
     
}
/// 授权角色
pub async fn grant_role(user:&NetSysUser,roleidlist:Option<Vec<i64>>)->Result<String,Error>{
  let db= &GLOBAL_DB.clone();
  let mut tx = db.acquire_begin().await.unwrap();



  let _ = entity_sysuser_role::NetUserRole::delete_by_column(&tx, "userid", user.id).await;


  
  if let Some(roleids)=roleidlist{
    let mut roles:Vec<NetUserRole>=Vec::new();
     for i in roleids{
      let role=  NetUserRole{
        id: IdInstance::next_id(),
        roleid: i,
        userid: user.id,
    };
    roles.push(role);
     }
     if roles.len()>0 {
      let _ = NetUserRole::insert_batch(db, &roles, roles.len().try_into().unwrap()).await;
      }
  }
  let _ = tx.commit().await;

  Ok("".to_string())
}

/// 授权扩展组织
pub async fn grant_extorg(user:&NetSysUser,extorgidlist:Option<Vec<NetUserExtOrg>>)->Result<String,Error>{
  let db= &GLOBAL_DB.clone();
  let mut tx = db.acquire_begin().await.unwrap();



  let _ = entity_sysuser_extorg::NetUserExtOrg::delete_by_column(&tx, "userid", user.id).await;


  
  if let Some(roleids)=extorgidlist{
    let mut exts:Vec<NetUserExtOrg>=Vec::new();
     for i in roleids{
      let mut ext=  i.clone();
      ext.userid=user.id;
      ext.id=IdInstance::next_id();
       exts.push(ext);
     }
     if exts.len()>0{

      let _ = NetUserExtOrg::insert_batch(db, &exts, exts.len().try_into().unwrap()).await;
     }
     
  }
  let _ = tx.commit().await;

  Ok("".to_string())
}