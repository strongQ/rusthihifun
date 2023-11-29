use rbatis::Error;

use crate::{model::common_model::NameInput, entity::entity_syspos::{NetSysPos, self}, GLOBAL_DB};



/// 获取职位列表
pub async fn get_pos(input:&NameInput)->Result<Vec<NetSysPos>,Error>{

     let mut name="";
    let mut code="";
    if let Some(value)=&input.name{
       name=value.as_str();
    }
    if let Some(value)=&input.code{
       code=value.as_str();
    }
    let db= &GLOBAL_DB.clone();
    let result=entity_syspos::get_pos(db, name, code).await;
    match result{
        Ok(pos)=>{
  
          
          return  Ok(pos);
        },
        Err(err)=>{
          return  Err(err);
        }
      }
}