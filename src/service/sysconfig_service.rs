



use crate::{GLOBAL_DB, entity::entity_sysconfig};
/// 获取配置值
pub async fn get_config_value(code:String)->rbatis::Result<String>{

    let db= &GLOBAL_DB.clone();
   let value=entity_sysconfig::get_value(db, &code).await?;
    
   Ok(value)
   
}