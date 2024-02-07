use crate::redis::RedisClient;
use crate::{
    publish_enr, start_discv5, NUMBER_OF_NODES, REDIS_KEY_INITIATOR_ENR, REDIS_KEY_READY_TO_TEST,
    REDIS_KEY_RELAY_ENR, REDIS_KEY_TARGET_ENR, REDIS_KEY_TEST_COMPLETED,
};
use discv5::Enr;
use std::net::{IpAddr, Ipv4Addr};

pub(crate) async fn run(mut redis: RedisClient) {
    let discv5 = {
        // Start discv5 server with `initiator_router`'s external ip address.
        let discv5 = start_discv5(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 20))).await;
        let relay_enr: Enr = redis.pop(REDIS_KEY_RELAY_ENR).await;
        discv5.add_enr(relay_enr).unwrap();
        discv5
    };

    // Publish local ENR
    publish_enr(&mut redis, REDIS_KEY_INITIATOR_ENR, discv5.local_enr()).await;

    redis
        .signal_and_wait(REDIS_KEY_READY_TO_TEST, NUMBER_OF_NODES)
        .await;

    // Run FINDNODE query. This triggers hole punching.
    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;
    let found_enrs = discv5.find_node(target_enr.node_id()).await.unwrap();
    // Check that `found_enrs` contains the target's ENR.
    if !found_enrs
        .iter()
        .any(|enr| enr.node_id() == target_enr.node_id())
    {
        panic!("target's ENR not found.");
    }
    println!("Hole punching has been done successfully.");

    redis
        .signal_and_wait(REDIS_KEY_TEST_COMPLETED, NUMBER_OF_NODES)
        .await;

    // Check DHT
    let entries = discv5.table_entries();
    if !entries.iter().any(|entry| entry.0 == target_enr.node_id()) {
        panic!("target's ENR not found in the DHT.");
    }
}
