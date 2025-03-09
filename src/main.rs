use chrono::prelude::*;
use sha2::{Digest, Sha256}; // For hashing // For timestamps

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    previous_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &transactions);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            transactions,
        }
    }

    fn calculate_hash(
        index: u64,
        timestamp: i64,
        previous_hash: &str,
        transactions: &Vec<Transaction>,
    ) -> String {
        let input = format!("{}{}{}{:?}", index, timestamp, previous_hash, transactions);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}

struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"), vec![]);
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    fn add_transaction(&mut self, sender: String, receiver: String, amount: f64) {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };
        self.pending_transactions.push(transaction);
    }

    fn mine_block(&mut self) {
        if self.pending_transactions.is_empty() {
            println!("No transactions to mine.");
            return;
        }

        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            self.pending_transactions.clone(),
        );

        self.chain.push(new_block);
        self.pending_transactions.clear(); // Clear transactions after mining
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash
                != Block::calculate_hash(
                    current.index,
                    current.timestamp,
                    &current.previous_hash,
                    &current.transactions,
                )
            {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!("-------------------------------------------------");
            println!("Block Index: {}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Hash: {}", block.hash);
            println!("Transactions:");
            for tx in &block.transactions {
                println!("  {} -> {} : {} tokens", tx.sender, tx.receiver, tx.amount);
            }
        }
        println!("-------------------------------------------------");
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Adding transactions
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0);
    blockchain.add_transaction("Bob".to_string(), "Charlie".to_string(), 5.0);

    // Mining first block
    blockchain.mine_block();

    blockchain.add_transaction("Charlie".to_string(), "Alice".to_string(), 2.5);
    blockchain.mine_block();

    // Print the blockchain
    blockchain.print_chain();

    // Verify blockchain integrity
    println!("Blockchain valid? {}", blockchain.is_valid());
}
