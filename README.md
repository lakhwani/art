# Art House

Welcome to the Art House repository! This project is designed to create a decentralized platform for managing and trading digital art using blockchain technology with integration with RFID tags as an identifier and bridge to its real world asset. The repository contains two main components: a CosmWasm smart contract and an EVM (Ethereum Virtual Machine) smart contract.

## Repository Structure

The repository is organized into two main folders:

- **cosmwasm**: Contains the CosmWasm smart contracts and related code for the Art House project written in Rust.
- **evm-solidity**: Contains the Solidity smart contracts for deploying the Art House on Ethereum-compatible blockchains.

## Getting Started

### CosmWasm

To work with the CosmWasm contracts:

1. **Install Rust and CosmWasm:** Ensure you have Rust and CosmWasm installed on your machine.
2. **Build the Contracts:** Navigate to the `cosmwasm` directory and run `cargo build`.
3. **Run Tests:** Run `cargo test` to execute the unit and integration tests. Run `RUST_BACKTRACE=full cargo test` to see the entire backtrace.

### EVM-Solidity

To work with the Solidity contracts:

1. **Install Node.js and Hardhat:** Ensure you have Node.js and Hardhat installed on your machine.
2. **Install Dependencies:** Navigate to the `evm-solidity` directory and run `npm install`.
3. **Compile Contracts:** Run `npx hardhat compile` to compile the Solidity contracts.
4. **Run Tests:** Run `npx hardhat ignition deploy ignition/modules/ArtHouse.ts` to deploy the `ArtHouseBase.sol`.

## Contact

For questions or feedback, please send an email to `nikhilnlakhwani@gmail.com`
