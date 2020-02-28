extern crate chrono;
extern crate hyper;
extern crate local_ip;

use std::env;
use tokio::try_join;

mod backend_service;
mod distributor_service;
mod heartbeat_service;
mod logger;
mod models;
mod storage;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let fingerprint = args[1].clone();
    let name_node_url = args[2].clone();
    let port = str::parse::<u16>(&args[3].clone()).unwrap();

    let (sender, receiver) = tokio::sync::mpsc::channel(1024);
    let storage = storage::Storage::new();

    let distributor_fut =
        distributor_service::start(fingerprint.clone(), name_node_url.clone(), receiver);
    let heartbeat_fut = heartbeat_service::start(
        fingerprint.clone(),
        name_node_url.clone(),
        port.clone(),
        storage.clone(),
    );
    let backend_fut = backend_service::start(port.clone(), sender, storage.clone());

    let res = try_join!(distributor_fut, heartbeat_fut, backend_fut);

    match res {
        Ok(_) => println!("Shutdown"),
        Err(err) => {
            println!("[ERROR]\t{}", err);
        }
    }
}
