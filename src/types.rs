use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub value: u64,
    pub key: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    pub sender_key: [u8; 32],
    pub recipient_key: [u8; 32],
    pub transfer_value: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decrypt {
    pub key: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Withdraw {
    pub key: [u8; 32],
    pub value: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewResponse {
    pub result: u64,
}

pub const zero_key: [u8; 32] = [0; 32];