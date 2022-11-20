mod admin;

#[macro_use]
extern crate rust_i18n;

use std::env;
i18n!("locales");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    rust_i18n::set_locale(env::var("LANG").unwrap_or("en".to_string()).as_str());
    if let Err(e) = crate::admin::admin().await {
        println!("error: {}", e);
    }
}
