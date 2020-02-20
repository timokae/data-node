use crate::logger;
use crate::models;
use crate::storage;

use models::DataNode;
use storage::Storage;

use tokio::sync::mpsc;
use std::error::Error;

pub async fn start(mut receiver: mpsc::Receiver<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::log("Distributor", "Service started.");

    tokio::spawn(async move {
        logger::log("Distributor", "Waiting for message");

        while let Some(data) = receiver.recv().await {
            let msg = format!("Starting to distribute {}", data);
            logger::log("Distributor", &msg);

            // let nodes = all_nodes().await;
            // let nodes = vec![
            //     DataNode {
            //         address: String::from("192.168.0.100"),
            //         fingerprint: String::from("a"),
            //     },
            //     DataNode {
            //         address: String::from("192.168.0.101"),
            //         fingerprint: String::from("b"),
            //     },
            //     DataNode {
            //         address: String::from("192.168.0.102"),
            //         fingerprint: String::from("c"),
            //     },
            //     DataNode {
            //         address: String::from("192.168.0.103"),
            //         fingerprint: String::from("d"),
            //     },
            // ];
            // println!("{:?}", nodes);
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