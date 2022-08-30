#![warn(missing_docs)]
//! A transaction system that handles different types of transactions
//! and keeps a track of the Accounts involved in the transaction.

mod error;
mod entity;
mod service;
mod traits;

pub use error::{TransactionError, Result};
pub use entity::{TransactionType, TransactionRecord, TransactionEntry, Account};
pub use service::TransactionService;
pub use traits::Transaction;




