use salvo::Router;

use crate::controller::user_controller;




/// 不需要验证的路由
pub fn init_router_no_token() -> Router{
    let router=Router::new();
    router.push(Router::with_path("/api/sysAuth/captcha").get(user_controller::get_captcha))
    .push(Router::with_path("/api/sysAuth/loginConfig").get(user_controller::get_login_config))
    .push(Router::with_path("/api/sysAuth/login").post(user_controller::login))
    .push(Router::with_path("/api/sysAuth/logout").post(user_controller::log_out))
    
    
}

/// 需要jwt验证的路由
pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/api/sysAuth/userInfo").get(user_controller::get_info))
        .push(Router::with_path("/api/sysOnlineUser/page").get(user_controller::get_online_user_page))
        .push(Router::with_path("/api/sysUser/page").get(user_controller::get_user_page))
}