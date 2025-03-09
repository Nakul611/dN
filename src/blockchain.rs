use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(self.chain.len() as u64, transactions, prev_hash);
        self.chain.push(new_block);
    }
}
