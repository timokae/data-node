extern crate actix_web;
extern crate chrono;
extern crate hyper;
extern crate local_ip;

use std::env;
use tokio::try_join;

mod backend_service;
mod heartbeat_service;
mod logger;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let fingerprint = args[1].clone();
    let port = args[2].clone();

    let heartbeat_fut = heartbeat_service::start(fingerprint, port.clone());
    let backend_fut = backend_service::start_server(port.clone());

    let res = try_join!(heartbeat_fut, backend_fut);

    match res {
        Ok(_) => println!("Shutdown"),
        Err(err) => {
            println!("[ERROR]\t{}", err);
        }
    }
}
