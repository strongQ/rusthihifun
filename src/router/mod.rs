
use salvo::catcher::Catcher;
use salvo::prelude::{CatchPanic, OpenApi};
use salvo::prelude::*;
use salvo::cors::{Cors, AllowOrigin, AllowHeaders, CorsHandler};
use salvo::http::Method;
use salvo::session::{SessionHandler, CookieStore};
use salvo::{Service, Router, serve_static::StaticDir};
use salvo::logging::Logger;
use crate::controller::{swagger_controller, common_controller};

pub mod user_router;
pub mod menu_router;
pub mod notice_router;
pub mod org_router;
pub mod pos_router;
pub mod role_router;

///初始化路由
pub fn init_router(cors:CorsHandler) -> Router{

    //静态文件
    let static_dir=Router::with_path("/static/<*path>").get(
        StaticDir::new(["static/"]).listing(true)
    );

 

   
    // controller router
    let router=Router::new().hoop(Logger::new()).hoop(CatchPanic::new())
    .push(static_dir)
    .push(user_router::init_router_no_token())
    .push(Router::new().hoop(common_controller::auth_token)
                 .push(user_router::init_router())
                 .push(menu_router::init_router())
                 .push(notice_router::init_router())
                 .push(org_router::init_router())
                 .push(pos_router::init_router())
                 .push(role_router::init_router())
     );


    let session_handler=SessionHandler::builder(CookieStore::new(), b"rusthihifun-rusthihifun-rusthihifun-rusthihifun-rusthihifun-2023").build().unwrap();

    let doc=OpenApi::new("后台接口文档","0.0.1").tags(["用户","通知","角色","菜单","部门","字典","岗位"]).merge_router(&router);

    let router=router.push(Router::new().hoop(session_handler)
    .push(Router::new().hoop(swagger_controller::auth_token).push(doc.into_router("/api-doc/openapi.json"))
    .push(Scalar::new("/api-doc/openapi.json").into_router("/scalar"))
    .push(RapiDoc::new("/api-doc/openapi.json").into_router("/swagger")))
    .push(Router::with_path("/swaggerLogin").post(swagger_controller::swagger_login)));


       
    let main = Router::with_hoop(cors)
    .push(router).options(handler::empty());
    
    main
    
}

pub fn init_service() -> Service{
    // 跨域
    let cors_handler = Cors::new()
    .allow_origin(AllowOrigin::any())
  
    .allow_headers(AllowHeaders::any())
    .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
    .into_handler();
    let router = init_router(cors_handler.clone());
    Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err).hoop(cors_handler))
}