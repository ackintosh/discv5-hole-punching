use std::net::{IpAddr, Ipv4Addr};
use discv5::Enr;
use crate::{REDIS_KEY_RELAY_ENR, REDIS_KEY_TARGET_ENR, start_discv5};
use crate::redis::RedisClient;

pub(crate) async fn run(mut redis: RedisClient) {
    let external_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 20));
    let discv5 = start_discv5(external_ip).await;
    println!("aaaa");
    let relay_enr: Enr = redis.pop(REDIS_KEY_RELAY_ENR).await;
    println!("bbbb");
    discv5.add_enr(relay_enr).unwrap();

    let target_enr: Enr = redis.pop(REDIS_KEY_TARGET_ENR).await;
    println!("target_ip: {}", target_enr.ip4().unwrap());

    let r = discv5.find_node_designated_peer(target_enr, vec![0]).await;

    // tokio::time::sleep(Duration::from_secs(30)).await;
    println!("res: {:?}", r);
}
