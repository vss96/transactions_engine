/// Represents the Accounts of the clients transacting with the system.
pub struct Account {
    /// Unique identifier for the Client
    pub client: u16,
    /// Represents the available amount in the Account.
    pub available: f32,
    /// Represents the held amount in the Account.
    pub held: f32,
    /// Represents the total amount in the Account.
    pub total: f32,
    /// Boolean value to represent if the Account is locked or not.
    pub locked: bool,
}

/// All implementations for different transactions return a new Account
/// rather than mutating the existing account.
impl Account {
    /// Increments available and total amount for an account.
    pub fn deposit(&self, amount: f32) -> Self {
        Account {
            available: self.available + amount,
            total: self.total + amount,
            ..*self
        }
    }

    /// Decrements available and total amount for an account.
    pub fn withdrawal(&self, amount: f32) -> Self {
        Account {
            available: self.available - amount,
            total: self.total - amount,
            ..*self
        }
    }

    /// Decrements available balance by the amount disputed
    /// and holds the amount.
    pub fn dispute(&self, amount: f32) -> Self {
        Account {
            available: self.available - amount,
            held: self.held + amount,
            ..*self
        }
    }

    /// Disputed amount is reverted and returned back
    /// to the available balance.
    pub fn resolve(&self, amount: f32) -> Self {
        Account {
            available: self.available + amount,
            held: self.held - amount,
            ..*self
        }
    }

    /// Reverses the disputed transaction and locks Account.
    pub fn chargeback(&self, amount: f32) -> Self {
        Account {
            held: self.held - amount,
            total: self.total - amount,
            locked: true,
            ..*self
        }
    }

    /// Prints values of the account to STD.
    pub fn print(&self) {
        println!("{},{:.4},{:.4},{:.4},{}",
                 self.client,
                 self.available,
                 self.held,
                 self.total,
                 self.locked
        );
    }
}