mod initiator;
mod redis;
mod relay;
mod target;

use crate::redis::RedisClient;
use discv5::enr::{CombinedKey, EnrBuilder};
use discv5::{Discv5, ListenConfig};
use std::net::IpAddr;

// Redis key name to store ENRs.
const REDIS_KEY_RELAY_ENR: &str = "RELAY_ENR";
const REDIS_KEY_TARGET_ENR: &str = "TARGET_ENR";

// Redis keys to sync test sequence. These are used by `Redis::signal_and_wait()`.
const REDIS_KEY_READY_TO_TEST: &str = "READY_TO_TEST";
const REDIS_KEY_TEST_COMPLETED: &str = "TEST_COMPLETED";

// Number of nodes participating in this simulation.
const NUMBER_OF_NODES: u64 = 3;

#[tokio::main]
async fn main() {
    // Enable tracing.
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
        .expect("EnvFilter");
    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!(
            "Usage: {} <actor> \n <actor> possible values: initiator, relay, target",
            args.first().unwrap()
        );
    }

    // Redis client
    let redis = RedisClient::new().await;

    // TODO: clean up redis

    match args.get(1).unwrap().as_str() {
        "initiator" => initiator::run(redis).await,
        "relay" => relay::run(redis).await,
        "target" => target::run(redis).await,
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

async fn start_discv5(ip: IpAddr) -> Discv5 {
    let enr_key = CombinedKey::generate_secp256k1();

    let enr = EnrBuilder::new("v4")
        .ip(ip)
        .udp4(9000)
        .build(&enr_key)
        .expect("Construct local Enr");

    let mut discv5: Discv5 = Discv5::new(
        enr,
        enr_key,
        discv5::Discv5ConfigBuilder::new(ListenConfig::default()).build(),
    )
    .unwrap();

    discv5.start().await.expect("Start Discovery v5 server");
    discv5
}
