# 🚀 Ackee - School Of Solana S8 | Proof-of-Work Tasks Summary


<img width="2752" height="1536" alt="image" src="https://github.com/user-attachments/assets/848f6f35-6a36-4776-a167-4a6c4fd0dc7a" />



This repository summarizes the five classroom tasks completed for the Ackee School of Solana Season 8, designed to build a comprehensive foundation in Solana and Rust development.

---

## 📚 Core Tasks (1-5)

The curriculum progresses from theoretical foundations to practical Rust programming, on-chain development, and specialized concepts like security.

| Task | Title | Focus Areas | 
| :---: | :--- | :--- | 
| **1** | **Solana Core Concepts** | Solana architecture, principles, and ecosystem basics. | 
| **2** | **Rust Fundamentals** | Rust language features: structs, traits, error handling, and overflow safety in a Shapes/Calculator project. | 
| **3** | **On-Chain Vault Program** | Core Solana development: Program Derived Addresses (PDAs), Cross-Program Invocations (CPIs), and state management in an Anchor vault program. | 
| **4** | **Decentralized Twitter** | Building a complex Anchor dApp with instructions for creating tweets, adding/removing reactions, and adding/removing comments. | 
| **5** | **Solana Program Security** | Program security, common vulnerabilities, and runtime policies on the Solana blockchain. | 

---

## 🏆 Final Task: SolBrawl (Capstone Project)

| Detail | Description |
| :--- | :--- |
| **Project Title** | **SolBrawl** - A decentralized, gas-free contest platform built on Solana. |
| **Type** | Capstone Project: Solana Program deployed to Devnet. |
| **Program ID** | `EyMxoUu8So5nRVsAewjuWDZM16wVHJEKAzLSHs8kuEr9` |
| **Frontend URL** | [https://sol-brawl-v1.vercel.app](https://sol-brawl-v1.vercel.app) |

---

### Core Features of SolBrawl

SolBrawl is designed to enable organizations to launch trustless competitions with zero friction for participants:

* **SOL-Based Prizes:** Uses direct SOL prizes held in a **transparent escrow** until multisig consensus is reached.
* **🔒 Built-in Escrow System:** Prize funds are automatically locked using **PDAs** with a time-locked recovery mechanism (30 days after the deadline).
* **⚖️ Multisig Judging:** Supports a configurable judge panel (up to 5 judges) with a **customizable approval threshold** for trustless, transparent winner consensus.
* **⛽ Gas Sponsorship:** Contest creators can optionally sponsor transaction fees, enabling **barrier-free participation** where users do not need SOL for gas.
* **📝 Submission Management:** Tracks URL-based submissions (one per participant) with update capability and timestamp tracking.

---
