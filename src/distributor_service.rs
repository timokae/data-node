use crate::logger;
use crate::models;

use models::DataNode;
use tokio::sync::mpsc;
use std::error::Error;

pub async fn start(mut receiver: mpsc::Receiver<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Distributor", "Service started.");

    tokio::spawn(async move {
        logger::log("Distributor", "Waiting for message");

        while let Some(i) = receiver.recv().await {
            logger::log("Distributor", &i);
            let nodes = all_nodes().await;
            println!("{:?}", nodes);
        }
    })
    .await
    .unwrap();

    Ok(())
}

async fn all_nodes() -> Result<Vec<DataNode>, Box<dyn Error + Send + Sync>>{
    let uri = "http://localhost:3000/data-nodes";

    let res = reqwest::Client::new()
        .get(uri)
        .send()
        .await?
        .json::<Vec<DataNode>>()
        .await?;

    Ok(res)
}