
use salvo::{endpoint, Request, Depot};

use crate::{model::{ user_model::LoginState, common_model::{ResObj, NameInput}}, utils::res::{Res, res_json_ok, res_json_err}, entity::entity_sysorg::NetSysOrg, service::org_service};



/// 机构列表
#[endpoint(
    tags("部门"),
    parameters(
        NameInput
    ),
    responses(
        (status_code = 200,body=ResObj<Vec<NetSysOrg>>,description ="机构列表")

    ),
)]
pub async fn get_org_list(req:&mut Request,depot: &mut Depot)->Res<Vec<NetSysOrg>>{
    let state = depot.get::<LoginState>("user_state").unwrap();
    let payload = req.parse_queries::<NameInput>().unwrap();

   let result= org_service::get_user_orgs(state.userid, state.accounttype, &payload).await;

   match result{
    Ok(orgs)=>{

        return  Ok(res_json_ok(Some(orgs)))
    },
    Err(err)=>{

        return  Err(res_json_err(err.to_string()))
    }
   }
    
   
}