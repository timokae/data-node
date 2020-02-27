use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ForeignHash {
    pub hash: String,
    pub addr: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Heartbeat {
    pub node: DataNode,
    pub hashes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct HeartbeatResponse {
    pub status: String,
    pub foreign_hashes: Vec<ForeignHash>,
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
