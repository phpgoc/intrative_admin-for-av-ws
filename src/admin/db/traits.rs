use crate::admin::tcp::TcpResponse;

#[async_trait::async_trait]
pub(crate) trait AsyncDbTrait: Sized {
    fn new() -> Self;
    async fn list_channels(&self) -> TcpResponse;
    // async fn list_channel_users(&self, channel_id: &str) -> TcpResponse;
    // async fn list_published_users(&self,channel_id: &str) -> TcpResponse;
    // async fn set_room_public(&self, channel_id: &str, is_public: bool)  ->TcpResponse;
    // async fn set_channel_user_auth(&self, channel_id: &str, user_name: &str, auth: bool) ->TcpResponse;
    // async fn kick_out(&self, channel_id: &str, user: &str) -> TcpResponse;
    async fn query(&self, channel_id: &str) -> TcpResponse;
}
