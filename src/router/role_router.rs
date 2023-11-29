use salvo::Router;

use crate::controller::role_controller;


/// 需要jwt验证的路由
pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/api/sysRole/list").get(role_controller::get_role_list)
    )
}