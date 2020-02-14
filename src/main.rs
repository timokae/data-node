extern crate chrono;
extern crate hyper;
extern crate local_ip;

use std::env;
use tokio::join;

mod heartbeat_service;
mod logger;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let fingerprint = args[1].clone();
    let port = args[2].clone();

    let heartbeat_fut = heartbeat_service::start_service(fingerprint, port);

    join!(heartbeat_fut);
}
