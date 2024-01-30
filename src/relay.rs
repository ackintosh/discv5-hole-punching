use crate::redis::RedisClient;
use crate::{
    publish_enr, start_discv5, NUMBER_OF_NODES, REDIS_KEY_READY_TO_TEST, REDIS_KEY_RELAY_ENR,
    REDIS_KEY_TARGET_ENR, REDIS_KEY_TEST_COMPLETED,
};
use discv5::Enr;
use std::net::{IpAddr, Ipv4Addr};

pub(crate) async fn run(mut redis: RedisClient) {
    // Start discv5 server
    let discv5 = start_discv5(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 30))).await;

    // Publish local ENR
    publish_enr(&mut redis, REDIS_KEY_RELAY_ENR, discv5.local_enr()).await;

    redis
        .signal_and_wait(REDIS_KEY_READY_TO_TEST, NUMBER_OF_NODES)
        .await;

    // Make sure that the DHT contains the target's ENR.
    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;
    let dht = discv5.table_entries();
    if !dht.iter().any(|entry| entry.0 == target_enr.node_id()) {
        panic!("target's ENR not found in the DHT.");
    }

    redis
        .signal_and_wait(REDIS_KEY_TEST_COMPLETED, NUMBER_OF_NODES)
        .await;

    println!("Test completed successfully.");
}
