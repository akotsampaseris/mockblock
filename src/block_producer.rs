use ethereum_types::{Address, H256, U256};

use crate::state::State;
use crate::mempool::Mempool;
use crate::types::{Block, Transaction};
use crate::utils::current_unix_timestamp;

/// Constants
const BLOCK_GAS_LIMIT: u64 = 10_000_000;
const BLOCK_BASE_FEE: u64 = 1_000;
const MOCK_MINER: Address = Address::repeat_byte(0xff);


pub fn produce_block(
    block_number: u64,
    parent_hash: H256,
    mempool: &mut Mempool,
    state: &mut State,
) -> Block {
    let mut included_tx: Vec<Transaction> = Vec::new();
    let mut gas_used = 0;

    while let Some(tx) = mempool.pop_front() {
        if gas_used + tx.gas > BLOCK_GAS_LIMIT {
            println!("Gas limit exceeded for tx: {:?}", tx);
            mempool.add_to_front(tx);
            break;
        }
        
        match state.apply_transaction(tx) {
            Ok(_) => {
                gas_used += tx.gas;
                included_tx.push(tx);
            }
            Err(err) => {
                println!("Error applying transaction: {}", err);
            }
        }
    }

    let mut block = Block {
        block_number,
        parent_hash,
        timestamp: current_unix_timestamp(),
        fee_recipient: MOCK_MINER,
        gas_limit: BLOCK_GAS_LIMIT,
        gas_used,
        transactions: included_tx,
        block_hash: None,
    };

    let block_hash = hash_block(&block);
    block.block_hash = Some(block_hash);
    block
}

fn hash_block(block: &Block) -> H256 {
    use sha3::{Digest, Keccak256};

    let mut hasher = Keccak256::new();
    hasher.update(&block.block_number.to_be_bytes());
    hasher.update(&block.timestamp.to_be_bytes());
    hasher.update(&block.parent_hash.as_bytes());
    hasher.update(&block.gas_limit.to_be_bytes());
    hasher.update(&block.gas_used.to_be_bytes());

    for tx in &block.transactions {
        hasher.update(tx.from.as_bytes());
        hasher.update(tx.to.as_bytes());
        hasher.update(tx.value.as_u64().to_be_bytes());
        hasher.update(tx.nonce.to_be_bytes());
    }

    H256::from_slice(hasher.finalize().as_slice())
}