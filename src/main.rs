extern crate chrono;
extern crate hyper;
extern crate local_ip;

use std::env;
use tokio::try_join;

mod backend_service;
mod heartbeat_service;
mod distributor_service;
mod logger;
mod models;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let fingerprint = args[1].clone();
    let name_node_addr = args[2].clone();
    let port = str::parse::<u16>(&args[3].clone()).unwrap();

    let (sender, receiver) = tokio::sync::mpsc::channel(1024);

    let distributor_fut = distributor_service::start(receiver);
    let heartbeat_fut = heartbeat_service::start(fingerprint, name_node_addr.clone(), port.clone());
    let backend_fut = backend_service::start(port, sender);

    let res = try_join!(distributor_fut, heartbeat_fut, backend_fut);

    match res {
        Ok(_) => println!("Shutdown"),
        Err(err) => {
            println!("[ERROR]\t{}", err);
        }
    }
}
