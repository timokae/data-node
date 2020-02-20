use crate::logger;
use crate::models;
use crate::storage;

use models::Package;
use storage::Storage;

use warp::Filter;
use tokio::sync::mpsc;
use std::net::{SocketAddr, Ipv4Addr, IpAddr::V4};
use serde::Deserialize;

pub async fn start(port: u16, sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Backend", &format!("Server started on port {}", port));

    let distribute = warp::post()
        .and(warp::path("distribute"))
        .and(warp::body::content_length_limit(1024 * 16)) // Only accept bodies smaller than 16kb...
        .and(warp::body::json())
        .map(move |package: Package| {
            handle_distribute(sender.clone(), package)
        });

    #[derive(Deserialize)]
    struct GetQuery { hash: String };
    let lookup = warp::get()
        .and(warp::path("lookup"))
        .and(warp::query())
        .map(|q: GetQuery| {
            handle_lookup(q.hash)
        });




    let routes = warp::any().and(distribute.or(lookup));
    let addr = SocketAddr::new(V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    warp::serve(routes)
        .bind(addr)
        .await;

    Ok(())
}

fn handle_distribute(mut sender: mpsc::Sender<String>, package: Package) -> warp::reply::Json {
    let _ = sender.try_send(package.data.clone());
    let hash = Storage::current().insert(package.data.clone());
    warp::reply::json(&hash)
}

fn handle_lookup(hash: String) -> warp::reply::Json {
    let h = hash.parse::<u64>().unwrap();
    let value = Storage::current().get(h);
    warp::reply::json(&value)
}
