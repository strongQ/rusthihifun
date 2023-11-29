use salvo::Router;

use crate::controller:: org_controller;


/// 需要jwt验证的路由
pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/api/sysOrg/list").get(org_controller::get_org_list)
    )
}