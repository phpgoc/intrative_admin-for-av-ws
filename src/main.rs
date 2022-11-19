mod admin;
use crate::admin::AdminCommand;

#[tokio::main]
async fn main() {
    if let Err(e) = crate::admin::admin().await {
        println!("error: {}", e);
    }
}
