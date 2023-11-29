use salvo::endpoint;

use crate::{entity::entity_sysnotice, utils::res::{Res, res_json_ok}, model::common_model::ResObj};

/// 未读消息
#[endpoint(
    tags("通知"),
    responses(
      (status_code = 200,body=ResObj<Vec<entity_sysnotice::NetSysNotice>>,description ="未读消息")
    ),
  )]
  pub async fn get_unread_notices()->Res<Vec<entity_sysnotice::NetSysNotice>>{
     
    Ok(res_json_ok(None))
  }