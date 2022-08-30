use super::TransactionType;
use serde::Deserialize;

/// Represents the transaction for different clients.
#[derive(Debug, Deserialize)]
pub struct TransactionRecord {
    /// Represents the type of Transaction.
    #[serde(alias = "type")]
    pub _type: TransactionType,
    /// Unique id representing the client.
    pub client: u16,
    /// Unique id representing the transaction.
    pub tx: u32,
    /// Amount pertaining to the transaction.
    /// It is only populated for `TransactionType::DEPOSIT`
    /// and `TransactionType::WITHDRAWAL`.
    pub amount: Option<f32>,
}

/// Represents the entry used to keep track of transactions for
/// disputes and other transactions.
/// Internally we keep track of transactions where each tx
/// maps to a TransactionEntry.
pub struct TransactionEntry {
    /// Unique id representing the client.
    pub client: u16,
    /// Amount pertaining to the transaction.
    pub amount: f32,
}