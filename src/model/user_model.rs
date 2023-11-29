use rbatis::rbdc::DateTime;
use salvo::{prelude::ToSchema, oapi::ToParameters};
use serde::{Serialize, Deserialize};



/// 配置信息
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct ConfigRes{

    #[serde(rename="secondVerEnabled")]
   pub second_ver_enabled:bool,
    #[serde(rename="captchaEnabled")]
   pub captcha_enabled:bool,
    #[serde(rename="watermarkEnabled")]
   pub watermark_enabled:bool
}

/// 验证码返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct CaptchaRes{
  pub img:String,
  pub id:i64
}

/// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct LoginRes{
  #[serde(rename="accessToken")]
  pub token:String,
  #[serde(rename="refreshToken")]
  pub refresh_token:String
}

// 登录请求参数
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginReq{
  pub code:Option<String>,
  pub password:Option<String>,
  pub account:Option<String>,
  #[serde(rename="codeId")]
  pub code_id:i64
}

// 用户信息返回
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct LoginUserRes {
    pub account: String,
    pub realname:Option<String>,
    pub avatar: Option<String>,
    pub password: Option<String>,
    pub signature:Option<String>,
    pub orgid:i64,
    pub orgname:Option<String>,
    pub posname:Option<String>,
    pub buttons:Option<Vec<String>>
}
/// 登录状态数据
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct LoginState{
  pub userid:i64,
  pub accounttype:i32

}

// 在线用户信息返回
#[derive(Serialize, Deserialize, Clone, Debug,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct OnlineUserRes {
    pub userid: i64,
    pub username:Option<String>,
    pub realname: Option<String>,
    // 连接时间
    pub time: Option<DateTime>,
    pub ip:Option<String>,
    pub browser:Option<String>,
    pub os:Option<String>
}


#[derive(Debug,Serialize,ToParameters,Deserialize,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct PageUserInput{
  #[serde(rename="Page")]
  pub page:u32,
  #[serde(rename="PageSize")]
  pub pagesize:u32,
   pub field:Option<String>,
   pub order:Option<String>,
   pub descstr:Option<String>,

   pub account:Option<String>,
   pub realname:Option<String>,
   pub phone:Option<String>,
   #[serde(rename="OrgId")]
   pub orgid:i64
   
}

impl Default for PageUserInput {
    fn default() -> Self {
        Self { page: 1, pagesize: 10, field: Default::default(), order: Default::default(), descstr: Default::default(), account: Default::default(), realname: Default::default(), phone: Default::default(), orgid: Default::default() }
    }
}





