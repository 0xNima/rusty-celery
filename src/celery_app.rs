#![allow(non_upper_case_globals)]

use anyhow::Result;
use async_trait::async_trait;
use crate::prelude::*;
use env_logger::Env;
use tokio::time::{self, Duration};

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use crate::task::Signature;

pub async fn create_app() -> Result<std::sync::Arc<crate::Celery<crate::broker::AMQPBroker>>> {
    dotenv().ok();

    let my_app = crate::app!(
        broker = AMQPBroker { env::var("AMQP_ADDR").unwrap() },
        tasks = [
            add,
            // long_running_task,
            // bound_task,
        ],
        task_routes = [
            "buggy_task" => "buggy-queue",
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10),
    ).await?;

    Ok(my_app)
}

// // Demonstrates a long running IO-bound task. By increasing the prefetch count, an arbitrary
// // number of these number can execute concurrently.
// #[task(max_retries = 2)]
// pub async fn long_running_task(secs: Option<u64>) {
//     let secs = secs.unwrap_or(10);
//     time::sleep(Duration::from_secs(secs)).await;
// }

// // Demonstrates a task that is bound to the task instance, i.e. runs as an instance method.
// #[task(bind = true)]
// pub fn bound_task(task: &Self) {
//     // Print some info about the request for debugging.
//     println!("{:?}", task.request.origin);
//     println!("{:?}", task.request.hostname);
// }

// This generates the task struct and impl with the name set to the function name "add"
#[crate::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}