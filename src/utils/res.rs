use salvo::oapi::ToSchema;
use salvo::prelude::Json;

pub use crate::model::common_model::ResObj;


impl<T:ToSchema> ResObj<T> {
    pub fn ok(result: Option<T>)->Self{
      Self {
          code: 200,
          message: "访问成功".to_string(),
          result,
          r#type:"success".to_string()
      }
    }
    pub fn custom_code(code:i32,message:String) -> Self {
        if code==200{
         return  Self {
            code,
            message,
            result: None,
            r#type:"suucess".to_string()
        }

        }
        Self {
          code,
          message,
          result: None,
          r#type:"error".to_string()
      }
       
    }

    pub fn err(err:String)->Self{
        Self {
          code: 500,
          message: err,
          result: None,
          r#type:"error".to_string()
      }
    }
}

#[allow(dead_code)]
pub fn res_ok<T:ToSchema>(data:Option<T>)->ResObj<T>{
  ResObj::ok(data)
}

#[allow(dead_code)]
pub fn res_json_ok<T:ToSchema>(data:Option<T>)->Json<ResObj<T>>{
  Json(ResObj::ok(data))
  
}

#[allow(dead_code)]
pub fn res_err<T:ToSchema>(msg:String)->ResObj<T>{
  ResObj::err(msg)
}

#[allow(dead_code)]
pub fn res_json_err<T:ToSchema>(msg:String)->Json<ResObj<T>>{
  Json(ResObj::err(msg))
}

#[allow(dead_code)]
pub fn res_custom<T:ToSchema>(code:i32,msg:String)->ResObj<T>{
  ResObj::custom_code(code,msg)
}

#[allow(dead_code)]
pub fn res_json_custom<T:ToSchema>(code:i32,msg:String)->Json<ResObj<T>>{
  Json(ResObj::custom_code(code,msg))
}

#[allow(dead_code)]
pub type Res<T> = Result<Json<ResObj<T>>,Json<ResObj<()>>>;

