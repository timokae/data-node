use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Heartbeat {
    pub node: DataNode,
    pub hashes: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataNode {
    pub address: String,
    pub fingerprint: String,
}

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub data: String,
}
