use std::{sync::mpsc, thread};

mod api;
mod utility;

mod db;

use db::models::Friend;

#[tokio::main]
async fn main() {
    let (tx, tr) = mpsc::channel::<Friend>();

    tokio::spawn(async {
        api::functions::init(tx).await;
    });
}
