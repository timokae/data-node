use crate::logger;
use crate::models;
use crate::storage;

use models::Package;
use storage::Storage;

use serde::Deserialize;
use std::net::{IpAddr::V4, Ipv4Addr, SocketAddr};
use tokio::sync::mpsc;
use warp::Filter;

pub async fn start(
    port: u16,
    sender: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Backend", &format!("Server started on port {}", port));

    let distribute = warp::post()
        .and(warp::path("distribute"))
        .and(warp::body::content_length_limit(1024 * 16)) // Only accept bodies smaller than 16kb...
        .and(warp::body::json())
        .map(move |package: Package| handle_distribute(sender.clone(), package));

    let save = warp::post()
        .and(warp::path("save"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(move |package: Package| handle_save(package));

    #[derive(Deserialize)]
    struct GetQuery {
        hash: String,
    };
    let lookup = warp::get()
        .and(warp::path("lookup"))
        .and(warp::query())
        .map(|q: GetQuery| handle_lookup(q.hash));

    let routes = warp::any().and(distribute.or(lookup).or(save));

    let addr = SocketAddr::new(V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    warp::serve(routes).bind(addr).await;

    Ok(())
}

fn handle_distribute(mut sender: mpsc::Sender<String>, package: Package) -> warp::reply::Json {
    let hash = Storage::current().insert(package.data.clone());
    let _ = sender.try_send(package.data.clone());
    warp::reply::json(&hash)
}

fn handle_save(package: Package) -> warp::reply::Json {
    let hash = Storage::current().insert(package.data.clone());
    warp::reply::json(&hash)
}

fn handle_lookup(hash: String) -> warp::reply::Json {
    let h = hash.parse::<u64>().unwrap();
    let value = Storage::current().get(h);
    warp::reply::json(&value)
}
