
use redis::{ToRedisArgs, RedisResult, Client,Commands,FromRedisValue};

use crate::GLOBAL_REDIS;


/// 设置数据，包含过期时间
#[allow(dead_code)]
pub fn set_ex<K:ToRedisArgs,V:ToRedisArgs>(key:K,value:V,second:usize)->RedisResult<()>{
    let _:()=Client::set_ex (&mut GLOBAL_REDIS.clone(),key,value,second)?;
    Ok(())
}

/// 获取数据
#[allow(dead_code)]
pub fn get<T:FromRedisValue,K:ToRedisArgs>(key:K)->RedisResult<T>{
  let t:T = Client::get(&mut GLOBAL_REDIS.clone(),key)?;
  Ok(t)
}

/// 删除数据
#[allow(dead_code)]
pub fn del<K:ToRedisArgs>(key:K)->RedisResult<()>{
  let _:() = Client::del(&mut GLOBAL_REDIS.clone(),key)?;
  Ok(())
}