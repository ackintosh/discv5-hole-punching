use crate::redis::RedisClient;
use crate::{REDIS_KEY_RELAY_ENR, REDIS_KEY_TARGET_ENR};
use discv5::{Discv5, Enr};

pub(crate) async fn run(discv5: Discv5, mut redis: RedisClient) {
    redis.push(REDIS_KEY_RELAY_ENR, discv5.local_enr()).await;
    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;
    println!("target_ip: {}", target_enr.ip4().unwrap());
}
