

use idgenerator::IdInstance;
use rbatis::{Error, rbdc::db::ExecResult};
use salvo::oapi::extract::PathParam;

use crate::{GLOBAL_DB, utils::{md5::create_md5, webtoken::create_token}, model::{user_model::{LoginUserRes, LoginState, PageUserInput, ChangePwdReq}, common_model::{Page, IdInput}, common_const}, entity::{entity_sysuser::{self, NetSysUser, NetSysUserInput}, entity_sysuser_extorg::{NetUserExtOrg, self}, entity_sysuser_role}};
use crate::utils::redis;

use super::{menu_service, sysconfig_service, role_service};
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

pub async fn get_baseinfo(userid:i64)->Result<NetSysUser,String>{
    let db= &GLOBAL_DB.clone();
    let  result=NetSysUser::select_by_column(db, "id",userid).await;
    match result {
        Ok(users)=>{
            Ok(users[0].clone())

        },
        Err(err)=>{
            Err(err.to_string())
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


/// 获取用户扩展机构集合
pub async fn get_ext_orgs(id:PathParam<Option<i64>>)->Result<Vec<NetUserExtOrg>,Error>{

    let userid=id.into_inner().unwrap();
    let db= &GLOBAL_DB.clone();
     let result=entity_sysuser_extorg::get_ext_orgs(db,userid).await;
     result
       
  }

/// 重置密码
  pub async fn reset_pwd(input:IdInput)->Result<i32,Error>{
    let pwd= sysconfig_service::get_config_value(common_const::SYS_PASSWORD.to_string()).await?;

    let db= &GLOBAL_DB.clone();
    let  users= entity_sysuser::NetSysUser::select_by_column(db, "id", input.id).await?;

    let md5pwd= create_md5(pwd);

    let mut user=users[0].clone();

    user.password=Some(md5pwd);

    let result= entity_sysuser::NetSysUser::update_by_column(db,&user,"id").await?;
    return Ok(result.rows_affected as i32);

  }

  /// 修改密码
  pub async fn change_pwd(pwd:ChangePwdReq,userid:i64)->Result<i32,String>{

    if let (Some(old),Some(new))=(pwd.passwordold,pwd.passwordnew){

        let db= &GLOBAL_DB.clone();
    let  users= entity_sysuser::NetSysUser::select_by_column(db, "id", userid).await.unwrap();
    let md5pwd=create_md5(old);
    let mut user=users[0].clone();
    if user.password!=Some(md5pwd){
       return  Err("原始密码错误".to_string());
    }

    user.password=Some(create_md5(new));

   let result= NetSysUser::update_by_column(db, &user, "id").await;

   match result {
    Ok(exec)=>{
        return  Ok(exec.rows_affected as i32);

    },
    Err(err)=>{
        return Err(err.to_string());
    }
       
   }

    }

   return Err("密码有误".to_string());
    
  }


  /// 删除账号
  pub async fn delete(input:IdInput)->Result<ExecResult,Error>{
    let db= &GLOBAL_DB.clone();
    let mut tx = db.acquire_begin().await.unwrap();

    
    let _= entity_sysuser::NetSysUser::delete_by_column(&tx, "id", input.id).await;
    let _ = entity_sysuser_role::NetUserRole::delete_by_column(&tx, "userid", input.id).await;

    let result=NetUserExtOrg::delete_by_column(&tx, "userid", input.id).await;

    let _=tx.commit().await;

    result
  }

  /// 添加账号
  pub async fn add( input: NetSysUserInput)->Result<String,String>{
    
    let pwd= sysconfig_service::get_config_value(common_const::SYS_PASSWORD.to_string()).await;

    if let Err(err)=pwd{
        return Err(err.to_string());
    }

    let mut user=NetSysUser::default();
    user.account=input.account;
    user.phone=input.phone;
    user.birthday= input.birthday;
    user.nickname= input.nickname;
    user.orgid= input.orgid;
    user.posid= input.posid;
    user.realname=input.realname;
    

    let db= &GLOBAL_DB.clone();

   let userresult= NetSysUser::select_by_column(db, "account", user.account.clone()).await;

   if let Err(err)=userresult{
    return Err(err.to_string());
}
   let users=userresult.unwrap();
   if users.len()>0 {
      return  Err("存在相同的账号".to_string());
   }
   
   user.password=Some(create_md5(pwd.unwrap()));

   let id=IdInstance::next_id();
   user.id = id;

   let result=NetSysUser::insert(db, &user).await;


   

   match result {
    Ok(_exec) => {

         // 角色授权
      let _ = role_service::grant_role(&user,input.roleidlist).await;
      let _=role_service::grant_extorg(&user,input.extorgidlist).await;
      return Ok("".to_string());
    },
    Err(err) => return Err(err.to_string()),
}  

  }
  