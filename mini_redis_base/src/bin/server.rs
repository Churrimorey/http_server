#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, net::SocketAddr, sync::Mutex};

use mini_redis_base::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = mini_redis_base::DEFAULT_ADDR.parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::redis::base::RedisServiceServer::new(S {
        map: Mutex::new(HashMap::<String, String>::new()),
    })
    .run(addr)
    .await
    .unwrap();
}
