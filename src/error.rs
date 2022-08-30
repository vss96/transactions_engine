use thiserror::Error;

/// Encompasses the possible errors
/// that are possible while executing transactions.
#[derive(Error, PartialEq, Debug)]
pub enum TransactionError {
    /// Error for when there is a dispute request for a transaction that
    /// is already under dispute.
    #[error("Given transaction is already under dispute.")]
    DisputeAlreadyExists,
    /// Error for when withdrawals are made
    /// without sufficient available balance.
    #[error("Given clientId does not have funds.")]
    InsufficientFunds,
    /// Dummy error which is used to complete the match patterns
    /// for resolving/ chargeback disputes. Occurs if any other operations
    /// are used other than resolve/ chargeback
    /// in the `transaction_service::process_dispute` flow.
    #[error("Given transaction type is invalid")]
    InvalidOperation,
    /// Occurs during transactions where the client
    /// has not yet opened an account.
    #[error("Given clientId does not have an account.")]
    InvalidAccount,
    /// Occurs during Deposit/ Withdrawal if `TransactionRecord`
    /// does not have the amount specified.
    #[error("Give transaction record does not have the amount specified.")]
    MissingAmount,
    /// Occurs during the Dispute flow where the transaction marked for
    /// dispute/resolve/chargeback is non-existent.
    #[error("Given transaction does not exist.")]
    MissingTransaction,
    /// Error for when a transaction is tried on a locked account.
    #[error("Given account is locked, due to which the transaction has been declined.")]
    LockedAccount,
    /// Error for when resolve/chargeback is attempted for a transaction which
    /// is not disputed yet.
    #[error("Given transaction is not currently under dispute.")]
    TransactionNotDisputed,
}

/// Simplified Result type which uses TransactionError.
pub type Result<T> = std::result::Result<T, TransactionError>;