use anyhow::Result;
use celery::prelude::*;
use env_logger::Env;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use celery::celery_app::create_app;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let my_app = create_app().await.unwrap();

    my_app.display_pretty().await;
    my_app.consume_from(&["celery"]).await?;

    Ok(())
}

