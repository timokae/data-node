use crate::{models, logger};

use std::error::Error;
use std::thread;
use std::time::Duration;
use models::DataNode;


pub async fn start(
    fingerprint: String,
    name_node_addr: String,
    port: u16
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("HeartbeatService", "Service started.");

    tokio::spawn(async move {
        loop {
            let _ = send_heartbeat(fingerprint.clone(), name_node_addr.clone(), port.clone()).await;
            thread::sleep(Duration::from_secs(60 * 2));
        }
    })
    .await
    .unwrap();

    Ok(())
}

async fn send_heartbeat(
    fingerprint: String,
    name_node_addr: String,
    port: u16,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ip = local_ip::get().unwrap();
    let heartbeat = DataNode {
        address: format!("{}:{}", ip.to_string(), port),
        fingerprint: fingerprint,
    };

    let uri = format!("{}/heartbeat", name_node_addr);

    logger::log("HeartbeatService", "Sending heartbeat");

    let res = reqwest::Client::new()
        .post(&uri)
        .json(&heartbeat)
        .send()
        .await?;

    logger::log("HeartbeatService", &res.status().to_string());

    Ok(())
}
