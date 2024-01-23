use discv5::enr::{CombinedKey, EnrBuilder};
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let enr_key = CombinedKey::generate_secp256k1();
    let enr = EnrBuilder::new("v4")
        .ip(get_ip())
        .udp4(9000)
        .build(&enr_key)
        .expect("Construct an Enr");

    println!("{:?}", enr.ip4());
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
