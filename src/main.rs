mod admin;

#[macro_use]
extern crate rust_i18n;

use std::env;
use std::env::args;
i18n!("locales");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    rust_i18n::set_locale(env::var("LANG").unwrap_or("en".to_string()).as_str());
    if args().len() != 1 {
        if args().nth(1).unwrap() == "admin" {
            admin::tcp::tcp_server().await;
        } else if args().nth(1).unwrap() == "client" {
            admin::tcp::tcp_client().await.unwrap();
        }
        return;
    }
    if let Err(e) = crate::admin::admin().await {
        println!("error: {}", e);
    }
}
