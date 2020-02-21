use crate::storage;
use crate::{logger, models};

use models::{DataNode, Heartbeat};
use std::error::Error;
use std::thread;
use std::time::Duration;
use storage::Storage;

pub async fn start(
    fingerprint: String,
    name_node_addr: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Heartbeat", "Service started.");

    tokio::spawn(async move {
        loop {
            let _ = send_heartbeat(&fingerprint, &name_node_addr, port).await;
            thread::sleep(Duration::from_secs(20 * 1));
        }
    })
    .await
    .unwrap();

    Ok(())
}

async fn send_heartbeat(
    fingerprint: &str,
    name_node_addr: &str,
    port: u16,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ip = local_ip::get().unwrap();

    let node = DataNode {
        address: format!("{}:{}", ip.to_string(), port),
        fingerprint: String::from(fingerprint.clone()),
    };
    let heartbeat = Heartbeat {
        node: node,
        hashes: Storage::current().hashes().clone(),
    };

    let uri = format!("{}/heartbeat", name_node_addr);
    println!("{:?}", heartbeat);
    logger::log("Heartbeat", "Sending heartbeat");

    let res = reqwest::Client::new()
        .post(&uri)
        .json(&heartbeat)
        .send()
        .await?;

    logger::log("Heartbeat", &res.status().to_string());

    Ok(())
}
