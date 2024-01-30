use crate::redis::RedisClient;
use crate::{
    publish_enr, start_discv5, NUMBER_OF_NODES, REDIS_KEY_INITIATOR_ENR, REDIS_KEY_READY_TO_TEST,
    REDIS_KEY_RELAY_ENR, REDIS_KEY_TARGET_ENR, REDIS_KEY_TEST_COMPLETED,
};
use discv5::Enr;
use std::net::{IpAddr, Ipv4Addr};

pub(crate) async fn run(mut redis: RedisClient) {
    // Start discv5 server with `target_router`'s external ip address.
    let discv5 = start_discv5(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 40))).await;

    // Publish local ENR
    publish_enr(&mut redis, REDIS_KEY_TARGET_ENR, discv5.local_enr()).await;

    let relay_enr: Enr = redis.pop(REDIS_KEY_RELAY_ENR).await;
    let relay_node_id = relay_enr.node_id();

    // Send Ping to relay node to establish session.
    let _ = discv5.send_ping(relay_enr).await.unwrap();
    // Make sure that the DHT contains the relay's ENR.
    let entries = discv5.table_entries();
    if !entries.iter().any(|entry| entry.0 == relay_node_id) {
        panic!("relay's ENR not found in the DHT.");
    }

    redis
        .signal_and_wait(REDIS_KEY_READY_TO_TEST, NUMBER_OF_NODES)
        .await;

    redis
        .signal_and_wait(REDIS_KEY_TEST_COMPLETED, NUMBER_OF_NODES)
        .await;

    // Check DHT
    let initiator_enr: Enr = redis.pop(REDIS_KEY_INITIATOR_ENR).await;
    let entries = discv5.table_entries();
    if !entries
        .iter()
        .any(|entry| entry.0 == initiator_enr.node_id())
    {
        panic!("initiator's ENR not found in the DHT.");
    }

    println!("Test completed successfully.");
}
