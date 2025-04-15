# 🧱 Mockblock

[![Language](https://img.shields.io/badge/Rust-🦀%20Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/github/license/akotsampaseris/mockblock?style=flat-square)](./LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/akotsampaseris/mockblock?style=flat-square)](https://github.com/akotsampaseris/mockblock/stargazers)
[![Build](https://img.shields.io/github/actions/workflow/status/akotsampaseris/mockblock/ci.yml?label=build&style=flat-square)](https://github.com/akotsampaseris/mockblock/actions)
[![Rust Version](https://img.shields.io/badge/Rust-1.75%2B-blue?style=flat-square)](https://blog.rust-lang.org)
[![Made with ❤️](https://img.shields.io/badge/made%20with-%E2%9D%A4-red?style=flat-square)](https://github.com/akotsampaseris)

> A lightweight Ethereum-like mock blockchain simulator for local development, testing, and tooling — no node, no sync, just blocks.

---

## 🚀 Overview

**Mockblock** is a developer tool that simulates a simplified Ethereum-like blockchain locally. It mimics block production, transaction handling, and account state management — all in-memory, fast, and test-friendly.

This is perfect for:
- ⚙️ Testing smart contract SDKs or dApps
- 🧪 Simulating transaction flows in CI
- 🧰 Building Web3 tools without waiting for a testnet
- 🧠 Learning how blockchains work under the hood

---

## 🧠 Key Features

- ⛓️ Block production every N seconds (configurable)
- 🔁 Transaction pool + nonce/gas simulation
- 💸 Account state with balances and nonces
- 📡 Simple JSON-RPC/HTTP API for interaction
- ⚡ Fully in-memory — no disk, no network dependencies
- 🧾 Configurable genesis state