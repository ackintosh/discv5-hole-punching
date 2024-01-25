mod redis;
mod relay;
mod target;

use crate::redis::RedisClient;
use discv5::enr::{CombinedKey, EnrBuilder};
use discv5::{Discv5, ListenConfig};
use std::net::IpAddr;

const REDIS_KEY_RELAY_ENR: &str = "RELAY_ENR";
const REDIS_KEY_TARGET_ENR: &str = "TARGET_ENR";

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    println!("args: {:?}", std::env::args().collect::<Vec<_>>());

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!(
            "Usage: {} <actor> \n <actor> possible values: initiator, relay, target",
            args.first().unwrap()
        );
    }

    // Construct local ENR
    let enr_key = CombinedKey::generate_secp256k1();
    let enr = EnrBuilder::new("v4")
        .ip(get_ip())
        .udp4(9000)
        .build(&enr_key)
        .expect("Construct local Enr");
    println!("enr: {:?}", enr.ip4());

    // Start Discv5 server
    let mut discv5: Discv5 = Discv5::new(
        enr,
        enr_key,
        discv5::Discv5ConfigBuilder::new(ListenConfig::default()).build(),
    )
    .unwrap();
    discv5.start().await.expect("Start Discovery v5 server");

    // Redis client
    let redis = RedisClient::new().await;

    match args.get(1).unwrap().as_str() {
        "initiator" => todo!(),
        "relay" => relay::run(discv5, redis).await,
        "target" => target::run(discv5, redis).await,
        _ => panic!("Invalid actor"),
    }
}

fn get_ip() -> IpAddr {
    let interface = if_addrs::get_if_addrs()
        .unwrap()
        .iter()
        .find(|interface| !interface.is_loopback() && !interface.is_link_local())
        .expect("")
        .clone();

    interface.addr.ip()
}
