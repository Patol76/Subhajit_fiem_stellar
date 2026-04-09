# ⚽ Soroban Sports Betting Smart Contract

## 📌 Project Description
![Uploading image.png…]()


This project is a decentralized **sports betting smart contract** built using **Soroban (Stellar Smart Contracts)** and Rust.

It enables users to place bets on sports matches in a **trustless, transparent, and tamper-proof environment**, where all data is stored on-chain.

The contract is designed to be simple, scalable, and extendable for real-world betting applications.

---

## 🚀 What It Does

* Allows an **admin** to create a sports match (e.g., Team A vs Team B)
* Enables users to **place bets** on their chosen team
* Stores all betting data **securely on-chain**
* Allows the admin to **declare match results**
* Provides public access to:

  * Match details
  * All placed bets

---

## ✨ Features

* 🔐 **Decentralized Logic** – No central authority controlling funds
* 🧾 **On-Chain Transparency** – All bets are publicly verifiable
* 👤 **Admin Control** – Secure match creation & result declaration
* ⚡ **Low Fees** – Built on Stellar's efficient network
* 🧱 **Modular Design** – Easy to extend with advanced features
* 📊 **Data Accessibility** – Retrieve match and betting data anytime

---

## 🛠 Tech Stack

* **Rust** – Smart contract programming language
* **Soroban SDK** – Stellar smart contract framework
* **Stellar Blockchain** – Fast and low-cost decentralized network

---

## 📦 Smart Contract Functions

### 🔹 `init(admin: Address)`

Initializes the contract and sets the admin.

### 🔹 `create_match(team1: Symbol, team2: Symbol)`

Creates a new match between two teams.

### 🔹 `place_bet(user: Address, amount: i128, team: Symbol)`

Allows users to place a bet on a specific team.

### 🔹 `declare_result(result: Symbol)`

Admin declares the winning team.

### 🔹 `get_match()`

Returns current match details.

### 🔹 `get_bets()`

Returns all bets placed by users.

---

## 🔄 How It Works (Flow)

1. Admin initializes the contract
2. Admin creates a match
3. Users place bets on teams
4. All bets are stored on-chain
5. Admin declares the result
6. (Future) Winners receive payouts automatically

---

## 🔗 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CC5F4MQRLZLTRP36FSHAQYEI7HMNNZU7HGMXTU5TUANS253FDB5EOHCX
---

## 🧪 How to Run Locally

```bash
# Build contract
stellar contract build

# Deploy contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --source-account YOUR_ACCOUNT
```

---

## 🔮 Future Improvements

* 💰 Automatic payout system for winners
* 🪙 Token-based betting (XLM / custom tokens)
* 📊 Dynamic odds calculation
* ⚽ Multi-match support
* 🔐 Anti-fraud and validation mechanisms
* 🌐 Frontend integration (React + Wallet)

---

## ⚠️ Disclaimer

This project is for **educational purposes only**.
Sports betting may be regulated or restricted in certain regions.
Use responsibly and comply with local laws.

---

## ⭐ Support

If you like this project, give it a ⭐ on GitHub and share it!

---

