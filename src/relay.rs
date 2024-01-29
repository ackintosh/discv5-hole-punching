use crate::redis::RedisClient;
use crate::{
    get_ip, start_discv5, NUMBER_OF_NODES, REDIS_KEY_READY_TO_TEST, REDIS_KEY_RELAY_ENR,
    REDIS_KEY_TARGET_ENR, REDIS_KEY_TEST_COMPLETED,
};
use discv5::Enr;

pub(crate) async fn run(mut redis: RedisClient) {
    let discv5 = start_discv5(get_ip()).await;

    redis.push(REDIS_KEY_RELAY_ENR, discv5.local_enr()).await;
    redis.push(REDIS_KEY_RELAY_ENR, discv5.local_enr()).await;
    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;

    redis
        .signal_and_wait(REDIS_KEY_READY_TO_TEST, NUMBER_OF_NODES)
        .await;

    // Make sure that the DHT contains the target's ENR.
    let dht = discv5.table_entries();
    if !dht.iter().any(|entry| entry.0 == target_enr.node_id()) {
        panic!("target's ENR not found in the DHT.");
    }

    redis
        .signal_and_wait(REDIS_KEY_TEST_COMPLETED, NUMBER_OF_NODES)
        .await;

    println!("done");
    let e = discv5.table_entries();
    println!("{:?}", e);
}
