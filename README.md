# **Rust Staking Contract** 🚀

This project demonstrates how to interact with a **Solidity-based staking smart contract** using Rust. It allows users to **stake**, **unstake**, and check their **staked balance** using the `ethers-rs` library.

---

## **Table of Contents** 📑

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Setup](#setup)
4. [Usage](#usage)
5. [Smart Contract](#smart-contract)
6. [Contributing](#contributing)
7. [License](#license)

---

## **Overview** 🌟

This project is a **Rust-based CLI tool** that interacts with a staking smart contract deployed on a blockchain (e.g., Ethereum or a local testnet). It uses the `ethers-rs` library to send transactions and call methods on the smart contract.

### **Key Features**:
- **Stake tokens** into the contract.
- **Unstake tokens** from the contract.
- **Check the staked balance** of a wallet.

---

## **Prerequisites** 📋

Before running the project, ensure you have the following installed:

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
2. **Node.js**: Required if you're using a local blockchain like Hardhat or Ganache.
3. **Solidity Compiler (`solc`)**: Install via `npm install -g solc` or use a tool like Hardhat.
4. **Environment Variables**: Set up a `.env` file with the required variables (see [Setup](#setup)).

---
Install Dependencies:

Install Rust dependencies:

bash
Copy
cargo build
Set Up Environment Variables:

Create a .env file in the root directory and add the following variables:

env
Copy
RPC_URL="http://localhost:8545" # Replace with your Ethereum node URL
PRIVATE_KEY="your-private-key"   # Replace with your wallet's private key
CONTRACT_ADDRESS="0xYourContractAddress" # Replace with your deployed contract address
Deploy the Smart Contract:

If you haven't deployed the staking contract yet, deploy it using Hardhat, Remix, or another tool.

Update the CONTRACT_ADDRESS in the .env file with the deployed contract address.

ABI File:

Place the ABI JSON file (abi.json) in the src/ directory. This file is generated when you compile your Solidity contract.

Usage
Build the Project:

bash
Copy
cargo build
Run the Program:

bash
Copy
cargo run
The program will:

Stake 100 tokens.

Unstake 50 tokens.

Check and print the staked balance of the wallet.

Example Output:

Copy
Staked 100 tokens. Tx Hash: 0x...
Unstaked 50 tokens. Tx Hash: 0x...
Staked Balance: 50
Smart Contract
The Solidity smart contract used in this project should have the following methods:

solidity
Copy
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract StakingContract {
    mapping(address => uint256) public stakedBalances;

    function stake(uint256 amount) public {
        stakedBalances[msg.sender] += amount;
    }

    function unstake(uint256 amount) public {
        require(stakedBalances[msg.sender] >= amount, "Insufficient balance");
        stakedBalances[msg.sender] -= amount;
    }

    function getStakedBalance(address user) public view returns (uint256) {
        return stakedBalances[user];
    }
}
Deploy this contract to your blockchain network.

Compile the contract and save the ABI to src/abi.json.

Contributing
Contributions are welcome! If you'd like to contribute, please follow these steps:

Fork the repository.

Create a new branch for your feature or bugfix.

Commit your changes.

Submit a pull request.

License
This project is licensed under the MIT License. See the LICENSE file for details.

Acknowledgments
ethers-rs for the Rust Ethereum library.

Hardhat for local blockchain development.

<p style="font-family: 'Courier New'; font-size: 14px; color: gray;"> Made with ❤️ by Vignesh </p>

---
