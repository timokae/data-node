use hyper::{Client, Method, Request};

use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::logger;

#[derive(Serialize, Debug)]
struct Heartbeat {
    address: String,
    fingerprint: String,
}

pub async fn start(
    fingerprint: String,
    port: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("HeartbeatService started.");
    tokio::spawn(async move {
        loop {
            let _ = send_heartbeat(fingerprint.clone(), port.clone()).await;
            thread::sleep(Duration::from_secs(60 * 2));
        }
    })
    .await
    .unwrap();

    Ok(())
}

async fn send_heartbeat(
    fingerprint: String,
    port: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ip = local_ip::get().unwrap();
    let heartbeat = Heartbeat {
        address: format!("{}:{}", ip.to_string(), port),
        fingerprint: fingerprint,
    };
    let body = serde_json::to_string(&heartbeat).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:3000/heartbeat")
        .header("Content-Type", "application/json")
        .body(body.into())
        .unwrap();

    let client = Client::new();

    logger::log("Sending heartbeat");
    let resp = client.request(req).await?;
    logger::log(&resp.status().to_string());

    Ok(())
}
