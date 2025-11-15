use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Voter {
    pub id: u64,
    pub name: String,
    pub wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bounty {
    pub id: u64,
    pub project_id: u64,
    pub amount: u64,
    pub bounty_desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Active,
    Paused,
    Ready,
    Out,
}

