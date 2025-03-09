mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    // Create a sample transaction
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 50);

    // Mine a new block with this transaction
    blockchain.add_block(vec![tx]);

    // Print the blockchain
    println!("{:#?}", blockchain);
}
