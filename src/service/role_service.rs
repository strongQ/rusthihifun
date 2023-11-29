

use rbatis::Error;

use crate::{entity::entity_sysrole::{SysRoleOutput, self}, GLOBAL_DB};



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