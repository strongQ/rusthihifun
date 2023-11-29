




use rbatis::Error;

use crate::{utils::redis, model::{common_const::KEY_PERMISSION, menu_model::{SysMenuRes, SysMenuMeta}}, entity::entity_sysmenu, GLOBAL_DB};


/// 获取权限按钮
pub async fn get_menu_buttons(userid:i64,accounttype:i32)->Option<Vec<String>>{

   let data= redis::get::<String,String>(format!("{}{}",KEY_PERMISSION,userid));
   if let Ok(btnstr)=data{

    let btns=serde_json::from_str::<Vec<String>>(&btnstr).unwrap();
    return Some(btns);  
   }
   let db= &GLOBAL_DB.clone();
   // 获取按钮集合
   let result= entity_sysmenu::get_menu_permissions(db, userid, 3,accounttype).await;

   match result {
       Ok(permissions)=>{

       let mut menus:Vec<String>=Vec::new();
       
           for dto in permissions{
             menus.push(dto.permission.unwrap());
           }

            redis::set_ex(format!("{}{}",KEY_PERMISSION,userid), serde_json::to_string(&menus).unwrap(), 60).unwrap();
   
           return  Some(menus)
        
        

       },
       Err(err)=>{
         println!("查询权限按钮异常,{}" , err.to_string());
         None
       }
   }   
}
/// 获取所有菜单
pub async fn get_menus(userid:i64,accounttype:i32)->Result<Vec<SysMenuRes>,Error>{
   let db= &GLOBAL_DB.clone();

   let result=entity_sysmenu::get_menus(db, userid, accounttype).await;
   match result {
      Ok(menus)=>{

       let datas=  set_menu(menus, 0);


        return Ok(datas);
      },
      Err(err)=>{
         Err(err)
      }
       
   } 

}
/// 递归设置子菜单
fn set_menu(ori_arr:Vec<SysMenuRes>,pid:i64)->Vec<SysMenuRes>{
   let mut menulist=Vec::<SysMenuRes>::new();
   for  ori in ori_arr.iter(){
      if pid==ori.pid {
         let meta=Some(SysMenuMeta{
            title:ori.title.clone(),
            icon:ori.icon.clone(),
            isiframe:ori.isiframe,
            islink:ori.outlink.clone(),
            ishide:ori.ishide,
            iskeepalive:ori.iskeepalive,
            isaffix:ori.isaffix
      
         });
         let children=set_menu(ori_arr.clone(), ori.id);
         let mut menu=ori.clone();
         menu.meta=meta;
         if children.len()>0{
            menu.children=Some(children);
         }else{
            menu.children=None;
         }

         menulist.push(menu);
      }
   }

  menulist


   
}