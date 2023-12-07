

use std::collections::HashMap;
use std::vec;


use idgenerator::IdInstance;
use salvo::{endpoint, Depot, Response, Request};
use salvo::oapi::extract::{JsonBody, PathParam};

use crate::GLOBAL_DB;
use crate::entity::entity_sysconfig:: NetSysConfigDto;
use crate::entity::entity_sysuser::{NetSysUser, NetSysUserInput};
use crate::entity::entity_sysuser_extorg::NetUserExtOrg;
use crate::model::common_const::{SYS_WATERMARK, SYS_CAPTCHA, SYS_SECOND_VER};
use crate::model::common_model::{Page, IdInput};
use crate::model::user_model::{CaptchaRes, ConfigRes, LoginReq, LoginRes, LoginUserRes, LoginState, OnlineUserRes, PageUserInput, ChangePwdReq};
use crate::service::{sysconfig_service, user_service};
use crate::utils::captcha;
use crate::utils::res::{res_json_ok, res_json_err, res_json_custom};
use crate::{model::common_model::ResObj, utils::res::Res};
use crate::utils::redis;

 

/// 获取验证码
#[endpoint(
    tags("用户"),
    responses(
      (status_code = 200,body=ResObj<CaptchaRes>,description ="获取验证码")
    ),
  )]
  pub async fn get_captcha()->Res<CaptchaRes>{
    if let (captcha_str,Some(base64)) = captcha::create_captcha(){
      let id = IdInstance::next_id();
      // 验证码转小写
      redis::set_ex::<i64,String>(id, captcha_str.to_lowercase(), 300).unwrap();
      println!("code id is {}",id);
      Ok(res_json_ok(Some(CaptchaRes{img:base64,id})))
      
    }else{
      Err(res_json_err("验证码生成失败".to_string()))
    }
  }

  /// 获取配置信息
  #[endpoint(
    tags("用户"),
    responses(
      (status_code = 200,body=ResObj<ConfigRes>,description ="获取配置信息")
    ),
  )]
  pub async fn get_login_config()->Res<ConfigRes>{

    let db= &GLOBAL_DB.clone();
    let sql="select id,code,value from net_sysconfig where code in (?,?,?)";
   
    
    let waters:Vec<NetSysConfigDto>=db.query_decode(sql, vec![SYS_WATERMARK.into(),SYS_CAPTCHA.into(),SYS_SECOND_VER.into()]).await.unwrap_or_default();

 // 使用迭代器和 collect 方法来构造一个哈希表，避免多次遍历和匹配
 let config_map:HashMap<&str, bool> = waters.iter()
 .filter_map(|model| model.code.as_ref().zip(model.value.as_ref()))
 .map(|(code, value)| (code.as_str(), value.to_lowercase() == "true"))
 .collect();

// 使用哈希表的 get 方法来获取配置值，使用 unwrap_or 方法来提供默认值
let config:ConfigRes=ConfigRes{
 second_ver_enabled: *config_map.get(SYS_SECOND_VER).unwrap_or(&false),
 captcha_enabled: *config_map.get(SYS_CAPTCHA).unwrap_or(&false),
 watermark_enabled: *config_map.get(SYS_WATERMARK).unwrap_or(&false)
};
Ok(res_json_ok(Some(config)))         
  }
/// 退出登录
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="退出登录")
  ),
)]
pub async fn log_out(req:&mut Request)->Res<()>{
  if let Some(token) = req.headers().get("Authorization"){
    match redis::del(token.to_str().unwrap().to_string().replace("Bearer ","")){
      _=>Ok(res_json_ok(None))
    }
  }else{
    Ok(res_json_custom(401,"用户无权限".to_string()))
  }
}


  /// 登录
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<LoginRes>,description ="登录")
  ),
)]
pub async fn login(login_body:JsonBody<LoginReq>,res: &mut Response)->Res<LoginRes>{
 
  
  
 let value= sysconfig_service::get_config_value(SYS_CAPTCHA.to_string()).await.unwrap();

 if value.to_lowercase()=="true"{

  // 验证码
  if let Some(captcha_str) = login_body.code.clone(){
    let result = redis::get::<String,i64>(login_body.code_id);
    match result {
      Ok(captcha)=>{
        if captcha.is_empty() || !captcha_str.eq(&captcha){
          return  Err(res_json_err("验证码错误".to_string()));
         }else{
           redis::del(login_body.code_id).unwrap();   
         }

      },
      Err(err)=>{
        return  Err(res_json_err(err.to_string()));
      }
        
    }
    

  }else{
    return  Err(res_json_err("验证码错误".to_string()));
  } 
 }
 if let (Some(username),Some(password)) = (login_body.account.clone(),login_body.password.clone()){

  let result= user_service::validate_login_user(username, password).await;
 
  match result {
    Ok(token)=>{
      let refresh_token=token.clone();
      
      

      res.add_header("access-token", token.clone(), true).unwrap();
      res.add_header("x-access-token", token.clone(), true).unwrap();
      res.add_header("Access-Control-Expose-Headers", "access-token,x-access-token", true).unwrap();
    
     return  Ok(res_json_ok(Some(LoginRes{token,refresh_token})))

    },
    Err(msg)=>{

     return  Err(res_json_err(msg))
    }
      
  } 
  
}
return  Err(res_json_err("登录账号有误".to_string()));
}
/// 查看用户基本信息
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<NetSysUser>,description ="查看用户基本信息")
  ),
)]
pub async fn get_baseinfo(depot: &mut Depot)->Res<NetSysUser>{

  let state = depot.get::<LoginState>("user_state").unwrap();
  let result= user_service::get_baseinfo(state.userid).await;
  match result {
    Ok(user)=>{
     return  Ok(res_json_ok(Some(user)))

    },
    Err(msg)=>{

     return  Err(res_json_err(msg))
    }
      
  } 
}

/// 修改密码
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<i32>,description ="修改密码")
  ),
)]
pub async fn change_pwd(depot: &mut Depot,login_body:JsonBody<ChangePwdReq>)->Res<i32>{
  let state = depot.get::<LoginState>("user_state").unwrap();
  let pwd=login_body.into_inner();

  let result= user_service::change_pwd(pwd, state.userid).await;

  match result {
    Ok(user)=>{
     return  Ok(res_json_ok(Some(user)))

    },
    Err(msg)=>{

     return  Err(res_json_err(msg))
    }
      
  } 
}

/// 获取用户信息
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<LoginUserRes>,description ="获取登录账号")
  ),
)]
pub async fn get_info(depot: &mut Depot)->Res<LoginUserRes>{
  let state = depot.get::<LoginState>("user_state").unwrap();
  let result= user_service::get_login_info(state.userid, state.accounttype).await;

  match result {
    Ok(user)=>{
     return  Ok(res_json_ok(Some(user)))

    },
    Err(msg)=>{

     return  Err(res_json_err(msg))
    }
      
  } 
 
}

/// 用户列表
#[endpoint(
  tags("用户"),
  parameters(
    PageUserInput
  ),
  responses(
    (status_code = 200,body=ResObj<Page<NetSysUser>>,description ="用户列表")
  ),
)]
pub async fn get_user_page(req:&mut Request,depot: &mut Depot)->Res<Page<NetSysUser>>{

  let state = depot.get::<LoginState>("user_state").unwrap();
 
  let payload=req.parse_queries::<PageUserInput>().unwrap();
 
  let result= user_service::get_user_page(payload,state.userid,state.accounttype).await;

  match result{
    Ok(page)=>{

      Ok(res_json_ok(Some(page)))
    },
    Err(err)=>{

      Err(res_json_err(err.to_string()))
    }
  }
  
}


/// 在线用户用户列表
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<Page<OnlineUserRes>>,description ="在线用户列表")
  ),
)]
pub async fn get_online_user_page()->Res<Page<OnlineUserRes>>{
   
  Ok(res_json_ok(None))
}


/// 获取用户扩展机构集合
#[endpoint(
  tags("用户"),  
  responses(
      (status_code = 200,body=ResObj<Vec<NetUserExtOrg>>,description ="获取用户扩展机构集合")

  ),
)]
pub async fn get_ext_orgs(id:PathParam<Option<i64>>)->Res<Vec<NetUserExtOrg>>{
 let result=user_service::get_ext_orgs(id).await;

 match result{
  Ok(data)=>{

      
      return  Ok(res_json_ok(Some(data)))
  },
  Err(err)=>{

      return  Err(res_json_err(err.to_string()))
  }
 }
}

/// 重置密码
#[endpoint(
  tags("用户"),  
  responses(
      (status_code = 200,body=ResObj<i32>,description ="重置密码")

  ),
)]
pub async fn reset_pwd(login_body:JsonBody<IdInput>)->Res<i32>{
   let result=user_service::reset_pwd(login_body.into_inner()).await;
   match result{
    Ok(data)=>{
  
        
        return  Ok(res_json_ok(Some(data)))
    },
    Err(err)=>{
  
        return  Err(res_json_err(err.to_string()))
    }
   }
}

/// 重置密码
#[endpoint(
  tags("用户"),  
  responses(
      (status_code = 200,body=ResObj<String>,description ="删除账号")

  ),
)]
pub async fn delete(login_body:JsonBody<IdInput>)->Res<String>{
   let result=user_service::delete(login_body.into_inner()).await;
   match result{
    Ok(_data)=>{
  
        
        return  Ok(res_json_ok(Some("".to_string())))
    },
    Err(err)=>{
  
        return  Err(res_json_err(err.to_string()))
    }
   }
}


/// 添加账号
#[endpoint(
  tags("用户"),  
  responses(
      (status_code = 200,body=ResObj<String>,description ="添加账号")
  ),
)]
pub async fn add(login_body:JsonBody<NetSysUserInput>)->Res<String>{
  let input=login_body.into_inner();
 

   let result=user_service::add(input).await;
   match result{
    Ok(data)=>{
  
        
        return  Ok(res_json_ok(Some(data)))
    },
    Err(err)=>{
  
        return  Err(res_json_err(err.to_string()))
    }
   }
}


