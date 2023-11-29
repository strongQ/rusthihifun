

use rbatis::Error;

use crate::{GLOBAL_DB, utils::{md5::create_md5, webtoken::create_token}, model::{user_model::{LoginUserRes, LoginState, PageUserInput}, common_model::Page}, entity::entity_sysuser::{self, NetSysUser}};
use crate::utils::redis;

use super::menu_service;
/// 验证登录状态
pub async fn validate_login_user(account:String,password:String)->Result<String,String>{

    let db= &GLOBAL_DB.clone();
   
    let result=entity_sysuser::get_userdto(db, &account).await;

    
       match  result {
          Ok(user)=>{
            if user.status==2{
                return  Err("账号停用中".to_string());
               }
            
            match &user.password {
                Some(pwd)=>{
                       // 密码验证
                    let md5_password= create_md5(password);        
                    if !pwd.eq(&md5_password){
                        return  Err("密码不正确".to_string())
                    }
                },
                _=>()     
            }
            match create_token(user.id,user.account) {
                Ok(token)=>{
                    // userid*accounttype               
                   let state= serde_json::to_string(&LoginState{userid:user.id,accounttype:user.accounttype}).unwrap();
                  redis::set_ex(&token,state,3600).unwrap();
                 return  Ok(token)
                },
                Err(_err)=>{
                 return  Err("token生成失败".to_string())
                }
            }
       
   

          },
          Err(err)=>{

            return  Err(err.to_string())
            
          }
           
       }

    

   
}

/// 获取用户详情
pub async fn get_login_info(userid:i64,accounttype:i32)->Result<LoginUserRes,String>{

    let db= &GLOBAL_DB.clone();
    let  result=entity_sysuser::get_userinfo(db, userid).await;

    match result {
        Ok(mut user)=>{
            if user.account.is_empty(){
                return  Err("没有获取到用户信息".to_string())
             }
             // 获取buttons信息
             let buttons= menu_service::get_menu_buttons(userid, accounttype).await.unwrap();
             user.buttons=Some(buttons);
         
             Ok(user)

        },
        Err(err)=>{
           return Err(err.to_string())
        }
    }
   
}
/// 获取用户分页
pub async fn get_user_page(input:PageUserInput,userid:i64,accounttype:i32)->Result<Page<NetSysUser>,Error>{
    let db= &GLOBAL_DB.clone();
    let mut account="";
    let mut realname="";
    let mut phone="";
    if let Some(value)=&input.account{
       account=value.as_str();
    }
    if let Some(value)=&input.realname{
       realname=value.as_str();
    }
    if let Some(value)=&input.phone{
        phone=value.as_str();
     }
   let result= entity_sysuser::get_user_page(db, account, realname, phone, input.pagesize, input.page, userid, accounttype, input.orgid).await;

   match result {
    Ok(users)=>{
        let total=entity_sysuser::get_user_total(db, account, realname, phone, userid, accounttype, input.orgid).await.unwrap();
        let totalpages=(total+input.pagesize as u64-1)/input.pagesize as u64;
       let page= Page::<NetSysUser>{
        items:users,
        total: total,
        page: input.page,
        pagesize: input.pagesize,
        totalpages: totalpages as u32,
        hasprevpage: input.page>1,
        hasnextpage: totalpages>input.page as u64,
       };
       Ok(page)

    },
    Err(err)=>{
        Err(err)
    }
       
   }

}