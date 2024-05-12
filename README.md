# Vaccine Supply Chain Smart Contract
This repository contains a Soroban smart contract for managing a vaccine supply chain. It enables the creation, updating, purchasing, and retrieval of vaccine batches, providing a secure and efficient solution for tracking vaccine distribution.

# Features
Create Batch: Create a new vaccine batch with details such as manufacturer, tester, production date, and expiration date.
Update Batch Status: Update the status and current location of a vaccine batch.
Purchase Vaccine: Allow customers to purchase vaccine batches.
Get Batch Information: Retrieve information about a vaccine batch.

# Project Structure
`contracts/:` Contains the Soroban smart contract for the vaccine supply chain. Each contract resides in its own directory.
`frontend/:` [Optional] Frontend libraries for interfacing with the smart contract.
`Cargo.toml:` Top-level workspace configuration for managing dependencies.
`README.md:` This README file providing information about the project.

# Getting Started
To use the vaccine supply chain smart contract, follow these steps:

1) Clone the Repository: Clone this repository to your local machine.
git clone https://github.com/your-username/vaccine-supply-chain.git

2) Navigate to Contract Directory: Enter the directory containing the contract.
cd vaccine-supply-chain/contracts/Vaccine

3) Compile the Contract: Compile the smart contract by targeting the WebAssembly platform using Soroban SDK.
cargo build --target wasm32-unknown-unknown --release

4) Deploy the Contract: Deploy the compiled contract to your desired blockchain network.
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/vaccine.wasm \
  --source alice \
  --network testnet

5) Interact with the Contract: Utilize provided functions to create, update, purchase, and retrieve vaccine batches as needed.

soroban contract invoke \
  --id CCOUC6YK6EQ3CCO3V2DQIJNSHNVG6XNXDPTCNQGK7HKVBD4TWG66MWLQ \
  --source alice \
  --network testnet \
  -- \
  create_batch \
  --id 1 \
  --manufacturer "aryan" \
  --tester "harsh" \
  --production_date "1998-01-21" \
  --expiration_date "1999-02-28" \
  --current_location "pune" \
  --from alice

# Contributing
 Contributions are encouraged! To contribute to this project, follow these steps:

1) Fork the repository.
2) Create your feature branch (git checkout -b feature/your-feature).
3) Commit your changes (git commit -am 'Add your feature').
4) Push to the branch (git push origin feature/your-feature).
5) Create a new pull request.
