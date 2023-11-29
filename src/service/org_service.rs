use rbatis::Error;

use crate::{GLOBAL_DB, entity::entity_sysorg::{self, NetSysOrg}, model::common_model::NameInput, };

/// 获取组织列表
pub async fn get_user_orgs(userid:i64,accounttype:i32,input:&NameInput)->Result<Vec<NetSysOrg>,Error>{

    let mut name="";
    let mut code="";
    if let Some(value)=&input.name{
       name=value.as_str();
    }
    if let Some(value)=&input.code{
       code=value.as_str();
    }
    let db= &GLOBAL_DB.clone();
    let  result=entity_sysorg::get_user_orgs(db, userid,  accounttype,name,code).await;

    match result{
      Ok(orgs)=>{

         let id=orgs[0].clone().pid;
         let endorgs= set_org(orgs,id );
        return  Ok(endorgs);
      },
      Err(err)=>{
        return  Err(err);
      }
    }

   
   
}


/// 递归设置组织
fn set_org(org_arr:Vec<NetSysOrg>,pid:i64)->Vec<NetSysOrg>{
   let mut orglist=Vec::<NetSysOrg>::new();
   for  ori in org_arr.iter(){
      if pid==ori.pid {
         
         let children=set_org(org_arr.clone(), ori.id);
         let mut org=ori.clone();
        
         if children.len()>0{
            org.children=Some(children);
         }else{
            org.children=None;
         }

         orglist.push(org);
      }
   }

  orglist


   
}