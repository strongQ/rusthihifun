use salvo::{Request, endpoint};

use crate::{ model::common_model::{ResObj, NameInput}, service::pos_service, utils::res::{Res, res_json_ok, res_json_err}, entity::entity_syspos::NetSysPos};


/// 机构列表
#[endpoint(
    tags("岗位"),
    parameters(
        NameInput
    ),
    responses(
        (status_code = 200,body=ResObj<Vec<NetSysPos>>,description ="岗位列表")

    ),
)]
pub async fn get_pos_list(req:&mut Request)->Res<Vec<NetSysPos>>{
    let payload = req.parse_queries::<NameInput>().map_or(NameInput{name:None,code:None},|v|v);

   let result= pos_service::get_pos( &payload).await;

   match result{
    Ok(pos)=>{

        return  Ok(res_json_ok(Some(pos)))
    },
    Err(err)=>{

        return  Err(res_json_err(err.to_string()))
    }
   }
}