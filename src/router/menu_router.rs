use salvo::Router;

use crate::controller::menu_controller;


/// 需要jwt验证的路由
pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/api/sysMenu/loginMenuTree").get(menu_controller::get_menu_list)
    )
}