use crate::redis::RedisClient;
use crate::REDIS_KEY_TARGET_ENR;
use discv5::Discv5;

pub(crate) async fn run(discv5: Discv5, mut redis: RedisClient) {
    redis.push(REDIS_KEY_TARGET_ENR, discv5.local_enr()).await;
}
