# DAO Voting with ZK Proofs Using Curve25519 Dalek

This project utilizes the Curve25519-dalek library for cryptographic operations on the Solana blockchain. It includes functionalities for ElGamal encryption, zero-knowledge proofs (zk-SNARKs), and more.

## Features

- **ElGamal Encryption:** Encrypt and decrypt messages using ElGamal encryption scheme.
- **Create Polls**: Start new polls with specific proposals.
- **Cast Votes**: Vote on proposals with zk-SNARK proof verification.
- **Summarize Votes**: Tally and display the results.
- **Close Polls**: End polls and archive results.
- **Reward System**: Reward users for participating in votes.

## Key Components

- **lib.rs**: Main program logic, including functions for creating polls, voting, summarizing, and closing elections.
- **state.rs**: Defines state structures such as `Election`, `User`, and `ChangableTokenAccount`.
- **errors.rs**: Custom error definitions used for handling various error cases.
- **constants.rs**: Contains constant values used throughout the program.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).
- **Solana CLI**: Install the Solana CLI tools by following the instructions [here](https://docs.solana.com/cli/install-solana-cli-tools).
- **Anchor**: Install Anchor by following the instructions [here](https://book.anchor-lang.com/chapter_2/installation.html).
- **Node.js**: Ensure you have Node.js and npm installed. You can download them from [here](https://nodejs.org/).

### Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/your-repo/dao_zk_proof_contract_using_curve_dalek.git
   cd dao_zk_proof_contract_using_curve_dalek

2. **Install dependencies**:
   ```sh
   npm install

3. **Build the project**:
   ```sh
   anchor build

## Deployment

1. **Deploy the program**:
   ```sh
   anchor deploy

2. **Verify the deployment**:
   Ensure the program ID in lib.rs matches the deployed program ID.

## Usage

1. **Initialize**:
   Set up your Solana environment by configuring your wallet and network settings.

2. **Run the program**:
   Execute the program using Solana CLI commands or scripts provided in the repository.

3. **Test the program**:
   Run the tests to ensure everything works correctly:
   ```sh
   anchor test  

### Dependencies

1. **Anchor**: Framework for Solana programs.

2. **Curve25519-dalek:** Library for elliptic curve cryptography operations.

3. **bls12_381**: Library for BLS12-381 elliptic curve operations.

### License

This project is licensed under the MIT License - see the LICENSE file for details.

### Acknowledgements

1. **Anchor**: For providing the framework to build Solana programs.

2. **Curve25519-dalek**: For providing robust elliptic curve cryptography operations.

### Contact

If you have any questions, feel free to reach out at [ritikbhatt020@gmail.com].
