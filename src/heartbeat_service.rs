use crate::storage;
use crate::{logger, models};

use models::{DataNode, Heartbeat, HeartbeatResponse};
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use storage::Storage;

pub async fn start(
    fingerprint: String,
    name_node_addr: String,
    port: u16,
    storage: Arc<Storage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Heartbeat", "Service started.");

    tokio::spawn(async move {
        loop {
            match send_heartbeat(&fingerprint, &name_node_addr, port, storage.clone()).await {
                Ok(res) => handle_heartbeat_response(res, storage.clone()),
                _ => {}
            }
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
    storage: Arc<Storage>,
) -> Result<HeartbeatResponse, Box<dyn Error + Send + Sync>> {
    let ip = local_ip::get().unwrap();
    let hashes = storage.hashes();

    let node = DataNode {
        address: format!("{}:{}", ip.to_string(), port),
        fingerprint: String::from(fingerprint.clone()),
    };
    let heartbeat = Heartbeat {
        node: node,
        hashes: hashes,
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

    let body = res.json::<HeartbeatResponse>().await?;

    Ok(body)
}

fn handle_heartbeat_response(response: HeartbeatResponse, storage: Arc<Storage>) {
    storage.insert_foreign(response.foreign_hashes);
}
