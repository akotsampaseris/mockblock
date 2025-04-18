use ethereum_types::{Address, H256, U256};


#[derive(Clone, Debug)]
pub struct AccountState {
    pub balance: U256, // Balance of wallet in wei
    pub nonce: u64, // Wallet's current nonce
}


#[derive(Clone, Copy, Debug)]
pub struct Transaction {
    pub from: Address, // 20-byte ETH sender's address
    pub to: Address, // 20-byte ETH receiver's address
    pub value: U256, // Transferred amount in wei
    pub nonce: u64, // Sender's current nonce
    pub gas: u64, // Transaction gas cost for applied tx
}


#[derive(Clone, Debug)]
pub struct Block {
    pub block_number: u64, // Block height
    pub timestamp: u64, // Unix timestamp
    pub parent_hash: H256, // Previous block hash
    pub transactions: Vec<Transaction>, // tx in block
    pub gas_limit: u64, // Upper limit of block gas
    pub gas_used: u64, // Gas sum for tx in block
    pub fee_recipient: Address, // 20-byte ETH miner's address
    pub block_hash: Option<H256>, // Hash of execution block
}