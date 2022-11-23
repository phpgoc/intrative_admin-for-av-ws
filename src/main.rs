mod admin;
mod utils;
#[macro_use]
extern crate rust_i18n;
extern crate core;

use crate::utils::LogInfo;
use std::env;
use std::env::args;
use tracing::log::error;

i18n!("locales");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::try_init_timed().unwrap();

    rust_i18n::set_locale(
        env::var("LANG")
            .unwrap_or_else(|_| "en".to_string())
            .as_str(),
    );
    if args().len() != 1 && args().nth(1).unwrap() == "admin" {
        admin::tcp::tcp_server().await;
        return;
    }
    admin::tcp::connect_tcp().await.unwrap();
    if let Err(e) = crate::admin::admin().await {
        error!(
            "{:?}",
            LogInfo {
                action: "tcp",
                user: "".to_string(),
                message: e.to_string()
            }
        );
    }
}
