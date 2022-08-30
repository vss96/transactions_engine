use serde::Deserialize;

/// An enum to represent the different types of
/// possible transactions in the system.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    /// Adds money to the existing Account or
    /// creates a new account with the amount specified.
    DEPOSIT,
    /// Withdraws the amount specified if it is available.
    WITHDRAWAL,
    /// Raises a dispute for any deposit or withdraw transaction
    /// that might have been erroneous.
    DISPUTE,
    /// Resolves existing disputes and reverts the money that was
    /// previously held.
    RESOLVE,
    /// Reverses the transaction under dispute and locks the account
    /// for further transactions.
    CHARGEBACK
}