use std::collections::HashMap;
use ethereum_types::{Address, U256};
use crate::types::{Transaction, AccountState};

#[derive(Clone, Debug)]
pub struct ChainState {
    accounts: HashMap<Address, AccountState>,
}

impl ChainState {
    // Creates empty state
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    // Initializes state from a predefined accounts map
    pub fn from_genesis(accounts: HashMap<Address, AccountState>) -> Self {
        Self { accounts }
    }

    // Returns account balance
    pub fn get_balance(&self, address: &Address) -> U256 {
        self.accounts.get(address).map_or(U256::zero(), |account| account.balance)
    }

    // Returns account nonce
    pub fn get_nonce(&self, address: &Address) -> u64 {
        self.accounts.get(address).map_or(0, |account| account.nonce)
    }

    // Applies a transaction to the chain state
    pub fn apply_transaction(&mut self, trx: &Transaction) -> Result<(), String> {
        let sender = self.accounts.get_mut(&trx.from)
            .ok_or("Sender not found")?;

        if sender.nonce != trx.nonce {
            return Err("Invanlid nonce".into());
        }

        if sender.balance < trx.value {
            return Err("Insufficient balance".into());
        }

        sender.balance -= trx.value; // Remove TRX value from sender's balance
        sender.nonce += 1; // Increment sender's nonce

        // Find or create receiver's account state
        let receiver = self.accounts.entry(trx.to).or_insert(AccountState {
            balance: U256::zero(),
            nonce: 0
        });

        receiver.balance += trx.value; // Add TRX value to receiver's balance

        Ok(())
    }

    pub fn insert_account(&mut self, addr: Address, account: AccountState) {
        self.accounts.insert(addr, account);
    }
}