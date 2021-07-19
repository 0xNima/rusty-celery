// #![allow(non_upper_case_globals)]

use anyhow::Result;
// use async_trait::async_trait;
// // use structopt::StructOpt;
// use celery::prelude::*;
// use env_logger::Env;
// use tokio::time::{self, Duration};

// extern crate dotenv;
// use dotenv::dotenv;
// use std::env;

// mod celery_app;
// use celery_app::{create_app};//, add, long_running_task, bound_task};

// // #[derive(Debug, StructOpt)]
// // #[structopt(
// //     name = "celery_app",
// //     about = "Run a Rust Celery producer or consumer.",
// //     setting = structopt::clap::AppSettings::ColoredHelp,
// // )]
// // enum CeleryOpt {
// //     Consume,
// //     Produce {
// //         #[structopt(possible_values = &["add", "bound_task", "long_running_task"])]
// //         tasks: Vec<String>,
// //     },
// // }

#[tokio::main]
async fn main() -> Result<()> {
//     dotenv().ok();
//     env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

//     // let opt = CeleryOpt::from_args();

//     let my_app = create_app().await.unwrap();

//     my_app.display_pretty().await;
//     my_app.consume_from(&["celery", "buggy-queue"]).await?;
//     my_app.consume().await?;
    
//     // match opt {
//     //     CeleryOpt::Consume => {
//     //         my_app.display_pretty().await;
//     //         my_app.consume_from(&["celery", "buggy-queue"]).await?;
//     //         my_app.consume().await?;
//     //     }
//     //     CeleryOpt::Produce { tasks } => {
//     //         if tasks.is_empty() {
//     //             // Basic task sending.
//     //             my_app.send_task(add::new(1, 2)).await?;
//     //             my_app.send_task(bound_task::new()).await?;

//     //             // Sending a task with additional options like `countdown`.
//     //             my_app.send_task(add::new(1, 3).with_countdown(3)).await?;

//     //             // Send the long running task that will fail with a timeout error.
//     //             my_app
//     //                 .send_task(long_running_task::new(Some(3)).with_time_limit(2))
//     //                 .await?;
//     //             // Send the long running task that will succeed.
//     //             for _ in 0..100 {
//     //                 my_app
//     //                     .send_task(long_running_task::new(Some(10)).with_time_limit(20))
//     //                     .await?;
//     //             }
//     //         } else {
//     //             for task in tasks {
//     //                 match task.as_str() {
//     //                     "add" => my_app.send_task(add::new(1, 2)).await?,
//     //                     "bound_task" => my_app.send_task(bound_task::new()).await?,
//     //                     "long_running_task" => {
//     //                         my_app.send_task(long_running_task::new(Some(3))).await?
//     //                     }
//     //                     _ => panic!("unknown task"),
//     //                 };
//     //             }
//     //         }
//     //         my_app.close().await?;
//     //     }
//     // };
    println!("Hello World");
    Ok(())
}
