# Description

## Solana Proof of History (PoH) Simulation in RustThis project is a simplified simulation of Solana’s Proof of History (PoH) implemented in Rust.
 It demonstrates how PoH creates a cryptographic hash chain to sequence transactions, providing a verifiable, tamper-resistant record of time and order. The simulation is designed for learning purposes, helping me understand Solana’s PoH mechanism while practicing Rust programming concepts like structs, ownership, and external crates.


## Overview
 This project simulates Solana’s Proof of History (PoH), a key component of Solana’s high-throughput blockchain. PoH generates a sequential SHA-256 hash chain to timestamp and order transactions, reducing network communication by enabling local verification. The simulation creates a PoH chain, embeds mock transactions, and verifies the chain’s integrity, mimicking Solana’s approach in a simplified way. It’s built in Rust to leverage its performance and safety features, making it an excellent learning tool for both Solana and Rust.


### What is Proof of History?
Proof of History (PoH) is a cryptographic technique used by Solana to create a verifiable sequence of hashes, acting as a decentralized clock. Each hash depends on the previous one, and transactions can be embedded into the chain to prove their order and timing. Key properties:
- Difficult to Produce: Generating the hash chain is computationally intensive (sequential hashing).
- Easy to Verify: Checking the chain’s integrity is fast, requiring only a few hash computations.
- Role in Solana: PoH orders transactions in the Transaction Processing Unit (TPU)’s PoH Stage, reducing communication overhead  and supporting Solana’s 50,000+ transactions per second (TPS).

### Project Features
- Generates a sequential SHA-256 hash chain starting with a “genesis” seed.
- Embeds mock transactions (as strings) into the chain, simulating Solana’s transaction inclusion.
- Supports empty “ticks” (hashes without transactions) to mark time passage.
- Verifies the chain’s integrity to demonstrate PoH’s tamper-resistant design.
- Outputs the chain’s contents (counter, hash, transaction) for easy visualization.
- Includes a tampering test to show verification failure if the chain is altered.


