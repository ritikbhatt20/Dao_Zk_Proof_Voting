# DAO Voting Program with ZK Proofs

This repository contains the implementation of a DAO voting program using Anchor, with optional privacy voting features using Zero-Knowledge (ZK) proofs. The project is structured into three main folders:

1. **Dao_Zk_Proof_Contract_Using_Bellman**
2. **Zk_Proof_Generation_For_Bellman**
3. **Dao_Zk_Proof_Contract_Using_Curve_Dalek**

## Folder Structure

### 1. Dao_Zk_Proof_Contract_Using_Bellman

This folder contains the smart contract implementation for the DAO voting program using Bellman ZK SNARKs.

- **Purpose**: Implement the DAO voting system with privacy voting using Bellman ZK SNARKs.
- **Features**: Voting system, results display, and optional ZK proof-based privacy voting.
- **Technologies**: Rust, Anchor, Bellman.

### 2. Zk_Proof_Generation_For_Bellman

This folder contains the implementation for generating ZK proofs and verification keys off-chain using the Bellman library.

- **Purpose**: Generate ZK proofs and verification keys off-chain to ensure privacy in the voting system.
- **Features**: ZK proof generation, verification key creation.
- **Technologies**: Rust, Bellman.

### 3. Dao_Zk_Proof_Contract_Using_Curve_Dalek

This folder contains the smart contract implementation for the DAO voting program using the Curve25519 Dalek library.

- **Purpose**: Implement the DAO voting system using the Curve25519 Dalek library.
- **Features**: Voting system, results display, optional privacy voting using Curve25519.
- **Technologies**: Rust, Anchor, Curve25519 Dalek.

## Getting Started

### Prerequisites

- Rust
- Anchor
- Solana CLI

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/ritikbhatt20/dao-voting-program.git
   cd dao-voting-program

2. Navigate to the desired folder (e.g., Dao_Zk_Proof_Contract_Using_Bellman) and follow the specific setup instructions in the respective README.md files.

### Usage

## Deploying Smart Contracts:

1. Navigate to the respective folder.
2. Follow the deployment instructions.

## Generating ZK Proofs:

1. Navigate to Zk_Proof_Generation_For_Bellman.
2. Follow the instructions to generate ZK proofs and verification keys.

### Contributing

1- **Fork the repository.**

2- **Create your feature branch (git checkout -b feature/fooBar).**

3- **Commit your changes (git commit -am 'Add some fooBar').**

4- **Push to the branch (git push origin feature/fooBar).**

5- **Create a new Pull Request.**

### License

Distributed under the MIT License. See LICENSE for more information.

### Acknowledgements

- [Anchor](https://github.com/project-serum/anchor)
- [Bellman](https://github.com/zkcrypto/bellman)
- [Curve25519 Dalek](https://github.com/dalek-cryptography/curve25519-dalek)

For any questions or feedback, feel free to open an issue or contact the maintainers.



