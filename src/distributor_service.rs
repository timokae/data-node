use crate::logger;
use crate::models;
// use crate::storage;

use models::{DataNode, Package};
// use storage::Storage;

use std::error::Error;
use tokio::sync::mpsc;

pub async fn start(
    fingerprint: String,
    name_node_url: String,
    mut receiver: mpsc::Receiver<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Distributor", "Service started.");

    tokio::spawn(async move {
        logger::log("Distributor", "Waiting for message");

        while let Some(data) = receiver.recv().await {
            let msg = format!("Starting to distribute {}", data);
            logger::log("Distributor", &msg);

            let nodes = all_nodes(&fingerprint, &name_node_url).await.unwrap();

            for node in nodes {
                logger::log("Distributor", &format!("Syncing with {}", node.fingerprint));
                let _ = send_to_node(node, data.clone()).await;
            }
        }
    })
    .await
    .unwrap();

    Ok(())
}

async fn all_nodes(
    fingerprint: &str,
    name_node_url: &str,
) -> Result<Vec<DataNode>, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/data-nodes?fingerprint={}", name_node_url, fingerprint);
    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json::<Vec<DataNode>>()
        .await?;

    Ok(res)
}

async fn send_to_node(node: DataNode, data: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("http://{}/save", node.address);
    let body = Package { data: data.clone() };

    let res = reqwest::Client::new().post(&url).json(&body).send().await?;
    logger::log(
        "Distributor",
        &format!("{} {}", node.fingerprint, res.status()),
    );

    Ok(())
}
