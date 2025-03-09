use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u32,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u32) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}
