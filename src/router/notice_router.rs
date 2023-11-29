use salvo::Router;

use crate::controller:: notice_controller;


/// 需要jwt验证的路由
pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/api/sysNotice/unReadList").get(notice_controller::get_unread_notices)
    )
}