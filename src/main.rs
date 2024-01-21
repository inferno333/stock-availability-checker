use dotenv::dotenv;
use std::sync::Arc;

use handler::SiteInfo;

mod alertor;
mod error;
mod handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let nv_3070_card = handler::Handler::new(
    //