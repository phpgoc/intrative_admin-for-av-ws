use crate::admin::structs_types::CommandResult;
use crate::admin::tcp::TcpResponse;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChannelInfo {
    pub(crate) auth: HashSet<String>,
    pub(crate) published: HashSet<String>,
    pub(crate) joined: HashSet<String>,
    pub(crate) is_public_room: bool,
}

#[derive(Clone)]
pub struct Dummy {
    data: Arc<HashMap<String, ChannelInfo>>,
}

#[async_trait::async_trait]
impl crate::admin::db::traits::AsyncDbTrait for Dummy {
    fn new() -> Self {
        let mut data = HashMap::new();
        for channel in 1..6 {
            let mut auth = HashSet::new();
            let mut published = HashSet::new();
            let mut joined = HashSet::new();
            for user in 1..10 {
                auth.insert(format!("user{}", user % 4));
                published.insert(format!("user{}", user % 2));
                joined.insert(format!("user{}", user));
            }
            data.insert(
                format!("channel{}", channel),
                ChannelInfo {
                    auth,
                    published,
                    joined,
                    is_public_room: true,
                },
            );
        }

        Dummy {
            data: Arc::new(data),
        }
    }

    async fn list_channels(&self) -> TcpResponse {
        TcpResponse::List(self.data.keys().map(|x| x.to_string()).collect())
    }
    async fn query(&self, channel_id: &str) -> TcpResponse {
        match self.data.get(channel_id) {
            Some(channel_info) => TcpResponse::Query(channel_info.clone()),
            None => TcpResponse::DbError,
        }
    }
}
