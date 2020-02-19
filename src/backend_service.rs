use crate::logger;
use crate::models;

use models::Package;

use warp::Filter;
use tokio::sync::mpsc;
use std::net::{SocketAddr, Ipv4Addr, IpAddr::V4};

pub async fn start(port: u16, sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Backend", &format!("Server started on port {}", port));

    let promote = warp::post()
        .and(warp::path("distribute"))
        .and(warp::body::content_length_limit(1024 * 16)) // Only accept bodies smaller than 16kb...
        .and(warp::body::json())
        .map(move |package: Package| {
            distribute(sender.clone(), package)
        });

    let addr = SocketAddr::new(V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    warp::serve(promote)
        .bind(addr)
        .await;

    Ok(())
}

fn distribute(mut sender: mpsc::Sender<String>, package: Package) -> warp::reply::Json {
    let _ = sender.try_send(package.data.clone());
    warp::reply::json(&package)
}