use crate::redis::RedisClient;
use crate::{
    start_discv5, NUMBER_OF_NODES, REDIS_KEY_READY_TO_TEST, REDIS_KEY_RELAY_ENR,
    REDIS_KEY_TARGET_ENR, REDIS_KEY_TEST_COMPLETED,
};
use discv5::Enr;
use std::net::{IpAddr, Ipv4Addr};

pub(crate) async fn run(mut redis: RedisClient) {
    let discv5 = {
        let external_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 20));
        let discv5 = start_discv5(external_ip).await;
        let relay_enr: Enr = redis.pop(REDIS_KEY_RELAY_ENR).await;
        discv5.add_enr(relay_enr).unwrap();
        discv5
    };
    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;

    redis
        .signal_and_wait(REDIS_KEY_READY_TO_TEST, NUMBER_OF_NODES)
        .await;

    // Run FINDNODE query. This triggers hole punching.
    let r = discv5.find_node(target_enr.node_id()).await.unwrap();
    println!("res: {:?}", r);

    redis
        .signal_and_wait(REDIS_KEY_TEST_COMPLETED, NUMBER_OF_NODES)
        .await;
}
