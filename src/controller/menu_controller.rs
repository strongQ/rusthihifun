

use salvo::{endpoint, Depot};

use crate::{model::{common_model::ResObj, menu_model::SysMenuRes, user_model::LoginState}, utils::res::{res_json_custom, Res, res_json_ok}, service::menu_service};




/// 菜单列表
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<Vec<SysMenuRes>>,description ="菜单列表")
    ),
)]
pub async fn get_menu_list(depot: &mut Depot)->Res<Vec<SysMenuRes>>{
  
    let state = depot.get::<LoginState>("user_state").unwrap();

    let result= menu_service::get_menus(state.userid, state.accounttype).await;
    
    match result{
        Ok(res)=>{

            return  Ok(res_json_ok(Some(res)))

        },
        Err(err)=>{

           return Err(res_json_custom(400,err.to_string()))
        }
    }
   
           
        
}