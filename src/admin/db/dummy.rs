use crate::admin::tcp::TcpResponse;
use serde::{Deserialize, Serialize};
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
    data: Arc<Mutex<HashMap<String, ChannelInfo>>>,
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
            data: Arc::new(Mutex::new(data)),
        }
    }

    async fn list_channels(&self) -> TcpResponse {
        TcpResponse::List(
            self.data
                .lock()
                .await
                .keys()
                .map(|x| x.to_string())
                .collect(),
        )
    }

    async fn list_channel_users(&self, channel_id: &str) -> TcpResponse {
        TcpResponse::List(
            self.data
                .lock()
                .await
                .get(channel_id)
                .unwrap()
                .joined
                .iter()
                .map(|x| x.to_string())
                .collect(),
        )
    }

    async fn set_room_public(&self, channel_id: &str, is_public: bool) -> TcpResponse {
        let mut data = self.data.lock().await;
        match data.get_mut(channel_id) {
            Some(t) => {
                t.is_public_room = is_public;
                TcpResponse::Ok
            }
            None => TcpResponse::UnknownSelected,
        }
    }

    async fn set_channel_user_auth(
        &self,
        channel_id: &str,
        user_name: &str,
        auth: bool,
    ) -> TcpResponse {
        let mut data = self.data.lock().await;
        match data.get_mut(channel_id) {
            Some(t) => {
                if auth {
                    t.auth.insert(user_name.to_string());
                } else {
                    t.published.remove(user_name);
                    t.auth.remove(user_name);
                }
                TcpResponse::Ok
            }
            None => TcpResponse::UnknownSelected,
        }
    }

    async fn kick_out(&self, channel_id: &str, user: &str) -> TcpResponse {
        let mut data = self.data.lock().await;
        match data.get_mut(channel_id) {
            Some(t) => {
                t.published.remove(user);
                TcpResponse::Ok
            }
            None => TcpResponse::UnknownSelected,
        }
    }

    async fn query(&self, channel_id: &str) -> TcpResponse {
        match self.data.lock().await.get(channel_id) {
            Some(channel_info) => TcpResponse::Query(channel_info.clone()),
            None => TcpResponse::DbError,
        }
    }
}
