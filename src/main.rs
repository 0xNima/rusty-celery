use actix_web::{get, web, App, HttpServer, Responder};
use teloxide::prelude::*;
use tokio::fs::File;
use dotenv::dotenv;
use futures::future::join_all;
use std::sync::*;
use celery::celery_app::{create_app, main_task};


// type Error = Box<dyn std::error::Error>;


#[get("/{sticker_name}")]
async fn handler(sticker_name: web::Path<std::string::String>) -> impl Responder {//}, bot: web::Data<teloxide::prelude::AutoSend<teloxide::Bot>>) -> impl Responder {
    let app = create_app().await.unwrap();
    app.send_task(main_task::new(sticker_name.into_inner())).await.unwrap();
    format!("{}", "ok")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // let bot = web::Data::new(Bot::from_env().auto_send());

    HttpServer::new(move || App::new()
        // .app_data(bot.clone())
        .service(handler))
        .bind("127.0.0.1:1700")?
        .run()
        .await?;

    Ok(())
}
