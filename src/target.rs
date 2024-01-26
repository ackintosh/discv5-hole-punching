use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use crate::redis::RedisClient;
use crate::{start_discv5, REDIS_KEY_RELAY_ENR, REDIS_KEY_TARGET_ENR};
use discv5::Enr;

pub(crate) async fn run(mut redis: RedisClient) {
    let external_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 40));
    let discv5 = start_discv5(external_ip).await;

    let relay_enr: Enr = redis.pop(REDIS_KEY_RELAY_ENR).await;
    println!("relay_ip: {}", relay_enr.ip4().unwrap());

    // Send Ping to relay node to establish session.
    let pong = discv5.send_ping(relay_enr).await.unwrap();
    println!("pong: {pong:?}");

    redis.push(REDIS_KEY_TARGET_ENR, discv5.local_enr()).await;

    println!("sleeping");
    tokio::time::sleep(Duration::from_secs(30)).await;
    println!("done");
}
