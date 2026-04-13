# 🌍 Travel NFTs on Stellar (Soroban)

## 📌 Project Description
Travel NFTs is a blockchain-based project built on Stellar's Soroban smart contract platform. It allows users to mint NFTs representing their travel experiences, such as visiting a specific location or landmark.

Each NFT acts as a digital collectible that stores travel-related metadata, making it a unique proof of journey.

---

## ⚙️ What it does
- Users can mint NFTs linked to travel destinations.
- Each NFT contains:
  - Owner name
  - Location visited
  - Metadata (e.g., description, date, memories)
- All NFTs are stored on-chain using Soroban smart contracts.

---

## ✨ Features
- 🌐 Decentralized NFT storage on Stellar
- 📍 Travel-based NFT minting
- 🧾 Simple metadata structure
- 🔎 Retrieve all minted NFTs
- ⚡ Built using Soroban smart contracts (Rust)

---

## 🔗 Deployed Smart Contract Link
https://stellar.expert/explorer/testnet/contract/CAWKC7G27TS4KIFVSUOUYERICYTODCJBBOIR4BJP2N4AGTQOIKCJKWXX
<img width="1920" height="1080" alt="Screenshot 2026-04-13 134316" src="https://github.com/user-attachments/assets/1df83209-4470-48d8-a7c2-0215ece88318" />

---

## 🚀 Future Improvements
- Add unique NFT IDs
- Enable transfer of NFTs between users
- Integrate IPFS for storing images
- Build a frontend UI (React / Next.js)
- Add authentication using Stellar wallets

---

## 🛠 Tech Stack
- Stellar Soroban
- Rust
- soroban-sdk

---

## 📦 How to Run

1. Install Soroban CLI  
2. Build the contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
