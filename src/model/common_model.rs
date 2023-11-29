use salvo::oapi::{ToSchema, ToParameters};
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,ToSchema)]
pub struct ResObj<T:ToSchema+'static>{
  pub code:i32,
  pub result:Option<T>,
  pub message:String,
  pub r#type:String
}

#[derive(Debug,Serialize,ToSchema)]
pub struct Page<T:ToSchema+'static>{
  pub items:Vec<T>,
  pub total:u64,
  pub page:u32,
  pub pagesize:u32,
  pub totalpages:u32,
  pub hasprevpage:bool,
  pub hasnextpage:bool
}

#[derive(Debug,Serialize,ToParameters,Deserialize,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct NameInput{
   pub name:Option<String>,
   pub code:Option<String>
   
}

#[derive(Debug,Serialize,ToParameters,Deserialize,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct BasePageInput{
  #[serde(rename="Page")]
  pub page:i32,
  #[serde(rename="PageSize")]
  pub pagesize:i32,
   pub field:Option<String>,
   pub order:Option<String>,
   pub descstr:Option<String>
   
}