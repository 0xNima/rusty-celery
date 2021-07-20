use anyhow::Result;
use async_trait::async_trait;
use crate::prelude::*;
use env_logger::Env;
use tokio::time::{self, Duration};

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use teloxide::prelude::*;
use teloxide::net::Download;

pub async fn create_app() -> Result<std::sync::Arc<crate::Celery<crate::broker::AMQPBroker>>> {
    dotenv().ok();

    let my_app = crate::app!(
        broker = AMQPBroker { env::var("AMQP_ADDR").unwrap() },
        tasks = [
            main_task,
        ],
        task_routes = [
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10),
    ).await?;

    Ok(my_app)
}


#[crate::task]
pub async fn main_task(sticker_name: std::string::String) {
    println!(">>>>>> {}", sticker_name);
    let bot = Bot::from_env().auto_send();
    let set = bot.get_sticker_set(sticker_name).send().await.unwrap();
    let mut tasks: Vec<_> = Vec::new();
    for _sticker in &set.stickers {
        tasks.push(download_sticker(_sticker, &bot));
    }
    tokio::join!(futures::future::join_all(tasks));
}


async fn download_sticker(sticker: &teloxide::types::Sticker, bot: &AutoSend<teloxide::Bot>) { //-> Result<(), Error> {
    let mut file = tokio::fs::File::create(format!("media/{}.webp", &sticker.file_unique_id)).await.unwrap();
    let dl = &bot.get_file(&sticker.file_id).send().await.unwrap();
    bot.download_file(&dl.file_path, &mut file).await.unwrap();
   
    println!("{:?}", dl.file_path);
    // Ok(())
}