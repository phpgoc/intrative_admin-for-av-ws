mod admin;

#[macro_use]
extern crate rust_i18n;
extern crate core;

use std::env;
use std::env::args;
i18n!("locales");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    rust_i18n::set_locale(
        env::var("LANG")
            .unwrap_or_else(|_| "en".to_string())
            .as_str(),
    );
    if args().len() != 1 {
        if args().nth(1).unwrap() == "admin" {
            admin::tcp::tcp_server().await;
        } else if args().nth(1).unwrap() == "client" {
            admin::tcp::connect_tcp().await.unwrap();
            admin::tcp::send_tcp_request(admin::tcp::TcpRequest::Ping);
        }
        return;
    }
    admin::tcp::connect_tcp().await.unwrap();
    if let Err(e) = crate::admin::admin().await {
        println!("error: {}", e);
    }
}
