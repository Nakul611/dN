# Rust Blockchain Documentation

## 1. Introduction
This document provides an overview of the Rust-based blockchain implementation, including its current functionality, architecture, and future development plans.

## 2. Purpose of This Blockchain
Before scaling the blockchain, it is essential to define why we are building it:

- **Decentralization**: A transparent, trustless ledger without intermediaries.
- **Efficiency & Security**: Utilizing Rust for high performance and memory safety.
- **Customizability**: Unlike Ethereum or Bitcoin, this blockchain will be optimized for custom smart contracts, low transaction fees, and security enhancements.
- **WASM-based Smart Contracts**: Future iterations will allow developers to write smart contracts in Rust or other languages that compile to WASM.

## 3. Current Features

### 3.1. Core Functionalities
- **Transactions**: Basic structure with sender, receiver, and amount.
- **Blocks**: Contain multiple transactions and link to previous blocks.
- **Blockchain**: A chain of blocks that stores transaction history.
- **Mining**: A simple block creation process.
- **Validation**: Ensures the integrity of the blockchain.

### 3.2. Code Structure
Currently, everything is in a single `main.rs` file. We will modularize this in the future.

#### 3.2.1. Transaction Struct
```rust
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}
```
- Represents a financial transaction.
- Includes sender, receiver, and amount.

#### 3.2.2. Block Struct
```rust
struct Block {
    index: u64,
    timestamp: i64,
    previous_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
}
```
- Contains transactions.
- Links to the previous block via `previous_hash`.
- `hash` is computed using SHA-256.

#### 3.2.3. Blockchain Struct
```rust
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}
```
- Stores all blocks.
- Manages pending transactions before mining.

#### 3.2.4. Functions
| Function | Purpose |
|----------|---------|
| `Block::new` | Creates a new block |
| `Block::calculate_hash` | Generates a hash using SHA-256 |
| `Blockchain::new` | Initializes the blockchain with a genesis block |
| `Blockchain::add_transaction` | Adds a transaction to the pending list |
| `Blockchain::mine_block` | Mines a new block and adds it to the chain |
| `Blockchain::is_valid` | Verifies blockchain integrity |
| `Blockchain::print_chain` | Displays the blockchain |

## 4. Running the Blockchain

### 4.1. Dependencies
Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
sha2 = "0.10"
```

### 4.2. Running the Program
Execute the following command:

```sh
cargo run
```

## 5. Future Development Roadmap

### Phase 1: Modularization
- Separate `block.rs`, `transaction.rs`, and `blockchain.rs`.
- Use Rust modules (`mod` and `pub`) to improve maintainability.

### Phase 2: Networking
- Implement a peer-to-peer (P2P) network using `tokio` or `libp2p`.
- Allow multiple nodes to sync the blockchain.

### Phase 3: Storage
- Use a database (e.g., RocksDB) for persistent blockchain storage.
- Implement state management for transactions.

### Phase 4: WASM Smart Contracts
- Integrate WASM-based contract execution.
- Create a virtual machine to execute contracts securely.

### Phase 5: Consensus Mechanism
- Implement Nominated Proof-of-Stake (NPoS) for validator selection.
- Optimize block finality using GRANDPA or BABE.

