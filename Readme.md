# Blockchain in Rust

## Overview
This is a simple blockchain implementation written in Rust. It includes basic features such as:
- Block and transaction structure
- Proof-of-existence with SHA-256 hashing
- Mining new blocks
- Validating blockchain integrity

The goal of this project is to create a modular and scalable blockchain that can be extended to support WASM-based smart contracts, networking, and consensus mechanisms.

## Features
- **Transactions:** Send tokens between users
- **Blocks:** Store transactions securely
- **Hashing:** Use SHA-256 to ensure block integrity
- **Mining:** Append new blocks to the chain
- **Validation:** Ensure the blockchain remains valid

## Prerequisites
Ensure you have Rust installed:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation
1. Clone this repository:
   ```sh
   git clone https://github.com/yourusername/rust-blockchain.git
   cd rust-blockchain
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```
3. Run the project:
   ```sh
   cargo run
   ```

## Project Structure
```
.
├── src
│   ├── main.rs         # Entry point
│   ├── blockchain.rs   # Blockchain logic
│   ├── block.rs        # Block structure
│   ├── transaction.rs  # Transaction handling
│   └── utils.rs        # Helper functions
├── Cargo.toml          # Rust dependencies
├── README.md           # Project documentation
└── .gitignore          # Ignore files
```

## Usage
- **Add transactions:** Create transactions between users
- **Mine a block:** Process transactions and add a block to the chain
- **Validate blockchain:** Check if the blockchain is intact

## Roadmap
✅ Basic blockchain implementation
🔲 Modularize code into separate files
🔲 Implement Proof-of-Stake (PoS)
🔲 Add networking for peer-to-peer communication
🔲 Integrate WASM-based smart contracts
🔲 Implement cross-chain communication

## Why This Blockchain?
The primary reason for building this blockchain is to create a **secure, scalable, and extensible** framework for decentralized applications. By making it modular, we ensure that future improvements like consensus mechanisms and smart contract execution can be seamlessly integrated.

## Contributing
Feel free to fork the repository and submit pull requests. Suggestions and improvements are welcome!

## License
This project is
