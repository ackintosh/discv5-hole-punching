use redis::aio::Connection;
use redis::{AsyncCommands, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

pub(crate) struct RedisClient {
    inner: Connection,
}

impl RedisClient {
    pub(crate) async fn new() -> Self {
        let client = Client::open("redis://redis:6379/").unwrap();
        let connection = client.get_async_connection().await.unwrap();

        RedisClient { inner: connection }
    }

    pub(crate) async fn push<V: Serialize + DeserializeOwned>(&mut self, key: &str, value: V) {
        let _: () = self
            .inner
            .rpush(key, serde_json::to_string(&value).unwrap())
            .await
            .unwrap();
    }

    pub(crate) async fn pop<V: DeserializeOwned>(&mut self, key: &str) -> V {
        let mut value = self
            .inner
            .blpop::<_, HashMap<String, String>>(key, 0_f64)
            .await
            .unwrap();
        serde_json::from_str(value.remove(key).unwrap().as_str()).unwrap()
    }
}
