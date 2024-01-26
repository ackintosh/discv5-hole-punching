use std::time::Duration;
use crate::redis::RedisClient;
use crate::{get_ip, REDIS_KEY_RELAY_ENR, REDIS_KEY_TARGET_ENR, start_discv5};
use discv5::Enr;

pub(crate) async fn run(mut redis: RedisClient) {
    let discv5 = start_discv5(get_ip()).await;

    redis.push(REDIS_KEY_RELAY_ENR, discv5.local_enr()).await;
    redis.push(REDIS_KEY_RELAY_ENR, discv5.local_enr()).await;
    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;
    println!("target_ip: {}", target_enr.ip4().unwrap());

    println!("sleeping");
    tokio::time::sleep(Duration::from_secs(30)).await;
    println!("done");
}
