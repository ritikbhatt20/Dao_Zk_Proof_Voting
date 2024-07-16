# ZK Proof Generation Using Bellman

This project demonstrates zero-knowledge proof (ZK proof) generation off-chain using the Bellman library. ZK proofs allow verifying computations without revealing inputs or intermediate values, ensuring privacy and integrity in decentralized applications.

## Features

- **ZK-SNARKs (Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge):** Generate succinct non-interactive proofs for arbitrary computations.
- **Sha256 Hashing**: Convert SHA256 hashes into cryptographic scalars for use in proofs.
- **Constraint System**: Define and enforce constraints to prove knowledge of private inputs without revealing them.

## Project Structure

- **src/main.rs**: Main entry point demonstrating ZK proof generation using SHA256 hashes and Bellman.
- **Cargo.toml**: Dependency configuration for Rust project and Bellman library.
- **Cargo.lock**: Detailed dependency tree lock file.

## Dependencies

- **Bellman**: Rust library for building zk-SNARK circuits and proofs.
- **BLS12-381**: Elliptic curve and field operations library for zk-SNARKs.
- **Rand**: Random number generation for cryptographic operations.
- **Sha2**: SHA-256 hashing algorithm for generating cryptographic hashes.

## Getting Started

### Prerequisites

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: Rust's package manager, included with Rust installation.
- **Bellman**: Include Bellman library in your Rust project dependencies.

### Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/your-repo/zk-proof-generation-using-bellman.git
   cd zk-proof-generation-using-bellman

2. **Install dependencies**:
   ```sh
   cargo build

## Usage

1. **Set up inputs in main.rs**:
   Replace your_docker_sha_here with your Docker SHA256 hash.
   Replace your_json_input_here with your JSON input hash.

2. **Run the program**:
   ```sh
   cargo run

3. **View generated proof and public inputs**:
   The program will output the generated proof in bytes and public inputs used.

### Dependencies

2. **Bellman**: zk-SNARK library for zero-knowledge proofs.

3. **bls12_381**: Library for BLS12-381 elliptic curve operations.

### License

This project is licensed under the MIT License - see the LICENSE file for details.

### Acknowledgements

1. **BLS12-381**: For enabling elliptic curve operations required for cryptographic computations.

2. **Bellman**: For providing the framework to build zk-SNARK circuits and generate proofs in Rust.

3. **Sha2**: For providing secure hashing algorithms necessary for input transformations.

### Contact

If you have any questions, feel free to reach out at [ritikbhatt020@gmail.com].
