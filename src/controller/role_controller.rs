use salvo::endpoint;

use crate::{model::common_model::{ResObj, NameInput}, entity::entity_sysrole::SysRoleOutput, utils::res::{Res, res_json_ok, res_json_err}, service::role_service};

/// 角色列表
#[endpoint(
    tags("角色"),
    parameters(
        NameInput
    ),
    responses(
        (status_code = 200,body=ResObj<Vec<SysRoleOutput>>,description ="角色列表")

    ),
)]
pub async fn get_role_list()->Res<Vec<SysRoleOutput>>{
   

   let result= role_service::get_roles().await;

   match result{
    Ok(role)=>{

        return  Ok(res_json_ok(Some(role)))
    },
    Err(err)=>{

        return  Err(res_json_err(err.to_string()))
    }
   }
}