


use idgenerator::{IdGeneratorOptions, IdInstance};
use once_cell::sync::Lazy;

use rbatis::RBatis;
use redis::Client;
use salvo::{Server, prelude::TcpListener, Listener};


mod utils;
mod router;
mod controller;
mod model;
mod entity;
mod service;



pub static GLOBAL_DB: Lazy<RBatis> = Lazy::new(|| RBatis::new());



pub static  GLOBAL_REDIS:Lazy<Client>=Lazy::new(|| {

    let client=redis::Client::open("redis://ip:port").expect("连接redis失败");
    client.get_connection().expect("连接redis失败");
    return  client;
});

#[tokio::main]
async fn main() {

    
   

   // Setup the option for the id generator instance.
   let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
   // Initialize the id generator instance with the option.
   // Other options not set will be given the default value.
   let _ = IdInstance::init(options).unwrap();

    // 连接数据库
    utils::postgresql::init_db().await;
   
    


    let service=router::init_service();
    
    Server::new(
        TcpListener::new("0.0.0.0:60070").bind().await
    ).serve(service).await;
   
}


