use std::collections::VecDeque;
use crate::types::Transaction;


/// A FIFO transaction memory pool
#[derive(Default)]
pub struct Mempool {
    queue: VecDeque<Transaction>
}

impl Mempool {
    /// Creates an empty mempool
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }

    /// Adds a transaction to the mempool
    pub fn add(&mut self, tx: Transaction) {
        self.queue.push_back(tx);
    }

    pub fn add_to_front(&mut self, tx: Transaction) {
        self.queue.push_front(tx);
    }

    /// Returns the number of transactions in the mempool
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Checks if the mempool is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the first transaction in the mempool
    pub fn first(&self) -> Option<&Transaction> {
        self.queue.front()
    }

    /// Returns the last transaction in the mempool
    pub fn last(&self) -> Option<&Transaction> {
        self.queue.back()
    }

    /// Removes the first transaction from the mempool
    pub fn pop_front(&mut self) -> Option<Transaction> {
        self.queue.pop_front()
    }

    /// Removes the last transaction from the mempool
    pub fn pop_back(&mut self) -> Option<Transaction> {
        self.queue.pop_back()
    }

    /// Returns an iterator over the transactions in the mempool
    pub fn iter(&self) -> impl Iterator<Item = &Transaction> {
        self.queue.iter()
    }

    /// Returns a read-only reference to the transactions in the mempool
    pub fn all(&self) -> &VecDeque<Transaction> {
        &self.queue
    }

    /// Clears all transactions from the mempool
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}