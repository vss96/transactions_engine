use crate::Result;


/// Defines the behaviour and possible operations you could
/// have in a transaction where the input record is of type T.
pub trait Transaction<T> {
    /// puts money into an account.
    fn deposit(&mut self, record: T) -> Result<()>;
    /// takes money away from an account.
    fn withdrawal(&mut self, record: T) -> Result<()>;
    /// Raises a dispute for one of the older transactions.
    fn dispute(&mut self, record: T) -> Result<()>;
    /// Ends an existing dispute and reverts the held money.
    fn resolve(&mut self, record: T) -> Result<()>;
    /// Reverts the transaction under dispute and locks the account
    fn chargeback(&mut self, record: T) -> Result<()>;
}