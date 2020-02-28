use crate::storage;
use crate::{logger, models};

use models::{DataNode, Heartbeat, HeartbeatResponse};
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use storage::Storage;

/// The Heartbeat Service starts a thread which send periodic messages over a http connection to the name server
///
/// * fingerprint: a unique identifier for this node
/// * name_node_addr: the address on which the name server can be reached, e.g. 'http://localhost:3000'
/// * port: the port on which the data node is listening for incoming http requests
/// * storage: the storage service
///
/// Example outgoing json object
/// ```
/// {
///     "node": {
///         "address": "192.168.0.100:8080",
///         "fingerprint": "client-1"
///     },
///     "hashes": [
///         "hash-1",
///         "hash-2"
///     ]
/// }
/// The hashes array contain all hashes the data node has saved, not the foreign hashes of other nodes.
/// ```

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
                Ok(res) => match res.json::<HeartbeatResponse>().await {
                    Ok(body) => handle_heartbeat_response(body, storage.clone()),
                    Err(_) => logger::log("Heartbeat", "Failed to decode heartbeat response"),
                },
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
) -> Result<reqwest::Response, Box<dyn Error + Send + Sync>> {
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
    logger::log("Heartbeat", "Sending heartbeat");

    let res = reqwest::Client::new()
        .post(&uri)
        .json(&heartbeat)
        .send()
        .await?;

    logger::log("Heartbeat", &res.status().to_string());

    Ok(res)
}

fn handle_heartbeat_response(response: HeartbeatResponse, storage: Arc<Storage>) {
    storage.insert_foreign(response.foreign_hashes);
}
