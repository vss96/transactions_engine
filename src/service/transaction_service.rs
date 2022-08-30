use std::collections::{HashMap, HashSet};
use crate::{Account, TransactionError, TransactionRecord, Result, TransactionType, Transaction, TransactionEntry};
use log::{info, error};


/// This service is responsible for implementing and handling
/// different types of transactions. Also keeps tracks the ongoing
/// transactions and accounts involved.
#[derive(Default)]
pub struct TransactionService {
    /// Keeps a track of all the Accounts in the system.
    account_ledger: HashMap<u16, Account>,
    /// Keeps a track of transactions related to deposits
    /// and withdrawals.
    transaction_ledger: HashMap<u32, TransactionEntry>,
    /// Keeps a track of all open disputes in the system.
    dispute_ledger: HashSet<u32>,
}

impl TransactionService {
    /// Takes in a `TransactionRecord` and processes it based on the
    /// transaction type.
    pub fn process(&mut self, record: TransactionRecord) -> Result<()> {
        info!("Processing transaction {} of type {:?} for client {}", record.tx, record._type, record.client);
        if let Some(_) = self.account_ledger.get(&record.client)
            .filter(|x| x.locked == true) {
            error!("Given transaction cannot occur since the Account is locked");
            return Err(TransactionError::LockedAccount);
        }

        match record._type {
            TransactionType::DEPOSIT => self.deposit(record),
            TransactionType::WITHDRAWAL => self.withdrawal(record),
            TransactionType::DISPUTE => self.dispute(record),
            TransactionType::RESOLVE => self.resolve(record),
            TransactionType::CHARGEBACK => self.chargeback(record),
        }
    }

    /// Generates the final output which displays different information
    /// about the Accounts that underwent the various transactions.
    pub fn generate_report(self) {
        println!("client,available,held,total,locked");
        self.account_ledger
            .into_values()
            .for_each(|acc| {
                acc.print();
            });
    }

    /// Common code pulled for Resolve and Chargeback. The only difference
    /// between the two is how the accounts are changed in the end.
    fn process_dispute(&mut self, record: &TransactionRecord) -> Result<()> {
        if !self.dispute_ledger.contains(&record.tx) {
            return Err(TransactionError::TransactionNotDisputed);
        }

        match self.transaction_ledger.get(&record.tx) {
            Some(t_entry) => {
                if record.client != t_entry.client {
                    return Err(TransactionError::MissingTransaction);
                }

                match self.account_ledger.get(&record.client) {
                    Some(account) => {
                        let updated_account = self.update_dispute(account, t_entry.amount, &record._type)?;
                        self.account_ledger.insert(record.client, updated_account);
                    }
                    None => {
                        return Err(TransactionError::InvalidAccount);
                    }
                }
                self.dispute_ledger.remove(&record.tx);
            }
            None => {
                return Err(TransactionError::MissingTransaction);
            }
        };

        Ok(())
    }
    fn update_dispute(&self, account: &Account, amount: f32, _type: &TransactionType) -> Result<Account> {
        match _type {
            TransactionType::RESOLVE => Ok(account.resolve(amount)),
            TransactionType::CHARGEBACK => Ok(account.chargeback(amount)),
            _ => Err(TransactionError::InvalidOperation)
        }
    }
}

impl Transaction<TransactionRecord> for TransactionService {
    fn deposit(&mut self, record: TransactionRecord) -> Result<()> {
        if let Some(amount) = record.amount {
            match self.account_ledger.get(&record.client) {
                Some(account) => {
                    let updated_account = account.deposit(amount);
                    self.account_ledger.insert(record.client, updated_account);
                }
                None => {
                    let account = Account {
                        client: record.client,
                        available: amount,
                        held: 0.0,
                        total: amount,
                        locked: false,
                    };
                    self.account_ledger.insert(record.client, account);
                }
            };
            self.transaction_ledger.insert(record.tx, TransactionEntry { client: record.client, amount });
            Ok(())
        } else {
            return Err(TransactionError::MissingAmount);
        }
    }

    fn withdrawal(&mut self, record: TransactionRecord) -> Result<()> {
        if let Some(amount) = record.amount {
            if let Some(_) = self.account_ledger.get(&record.client)
                .filter(|acc| acc.available - amount < 0.00) {
                return Err(TransactionError::InsufficientFunds);
            }

            match self.account_ledger.get(&record.client) {
                Some(account) => {
                    let updated_account = account.withdrawal(amount);
                    self.account_ledger.insert(record.client, updated_account);
                    self.transaction_ledger.insert(record.tx, TransactionEntry { client: record.client, amount: -amount });
                }
                None => {
                    return Err(TransactionError::InvalidAccount);
                }
            };
        } else {
            return Err(TransactionError::MissingAmount);
        }

        Ok(())
    }

    fn dispute(&mut self, record: TransactionRecord) -> Result<()> {
        if self.dispute_ledger.contains(&record.tx) {
            return Err(TransactionError::DisputeAlreadyExists);
        }

        match self.transaction_ledger.get(&record.tx) {
            Some(t_entry) => {
                if record.client != t_entry.client {
                    return Err(TransactionError::MissingTransaction);
                }

                if let Some(_) = self.account_ledger.get(&record.client)
                    .filter(|acc| acc.available - t_entry.amount < 0.00) {
                    return Err(TransactionError::InsufficientFunds);
                }

                match self.account_ledger.get(&record.client) {
                    Some(account) => {
                        let updated_account = account.dispute(t_entry.amount);
                        self.account_ledger.insert(record.client, updated_account);
                    }
                    None => {
                        return Err(TransactionError::InvalidAccount);
                    }
                }
                self.dispute_ledger.insert(record.tx);
            }
            None => {
                return Err(TransactionError::MissingTransaction);
            }
        };

        Ok(())
    }

    fn resolve(&mut self, record: TransactionRecord) -> Result<()> {
        self.process_dispute(&record)
    }

    fn chargeback(&mut self, record: TransactionRecord) -> Result<()> {
        self.process_dispute(&record)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_deposit_funds() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.5),
        };

        let result1 = service.process(record1);

        assert_eq!(Ok(()), result1);
        assert_eq!(1.5, service.account_ledger.get(&1).unwrap().available);
        assert_eq!(1.5, service.account_ledger.get(&1).unwrap().total);

        let record2 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(3.0),
        };


        let result2 = service.process(record2);

        assert_eq!(Ok(()), result2);
        assert_eq!(4.5, service.account_ledger.get(&1).unwrap().available);
        assert_eq!(4.5, service.account_ledger.get(&1).unwrap().total);
    }

    #[test]
    fn should_be_able_to_withdraw_from_account_with_funds() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let result1 = service.process(record1);

        assert_eq!(Ok(()), result1);
        assert_eq!(1.50, service.account_ledger.get(&1).unwrap().available);
        assert_eq!(1.50, service.account_ledger.get(&1).unwrap().total);

        let record2 = TransactionRecord {
            _type: TransactionType::WITHDRAWAL,
            client: 1,
            tx: 1,
            amount: Some(1.40),
        };


        let result2 = service.process(record2);
        assert_eq!(Ok(()), result2);
        assert_eq!("0.1000", format!("{:.4}", service.account_ledger.get(&1).unwrap().available));
        assert_eq!("0.1000", format!("{:.4}", service.account_ledger.get(&1).unwrap().total));
    }

    #[test]
    fn should_error_out_if_funds_are_insufficient() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::WITHDRAWAL,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let result1 = service.process(record1);

        assert_eq!(Err(TransactionError::InvalidAccount), result1);

        let record2 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.40),
        };

        let _ = service.process(record2);

        let record3 = TransactionRecord {
            _type: TransactionType::WITHDRAWAL,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let result3 = service.process(record3);

        assert_eq!(Err(TransactionError::InsufficientFunds), result3);
        assert_eq!(1.40, service.account_ledger.get(&1).unwrap().total);
        assert_eq!(1.40, service.account_ledger.get(&1).unwrap().available);
    }

    #[test]
    fn should_not_raise_dispute_for_invalid_transactions() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::DISPUTE,
            client: 1,
            tx: 2,
            amount: None,
        };
        let result = service.process(record2);
        assert_eq!(Err(TransactionError::MissingTransaction), result);
    }

    #[test]
    fn should_raise_dispute_for_valid_transaction() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::DISPUTE,
            client: 1,
            tx: 1,
            amount: None,
        };
        let result = service.process(record2);
        assert_eq!(Ok(()), result);
        let acc = service.account_ledger.get(&1).unwrap();
        assert_eq!(0.00, acc.available);
        assert_eq!(1.50, acc.total);
        assert_eq!(1.50, acc.held);
    }

    #[test]
    fn should_resolve_a_valid_dispute() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::DISPUTE,
            client: 1,
            tx: 1,
            amount: None,
        };
        let _ = service.process(record2);

        let record3 = TransactionRecord {
            _type: TransactionType::RESOLVE,
            client: 1,
            tx: 1,
            amount: None,
        };
        let result = service.process(record3);

        assert_eq!(Ok(()), result);
        let acc = service.account_ledger.get(&1).unwrap();
        assert_eq!(1.50, acc.available);
        assert_eq!(1.50, acc.total);
        assert_eq!(0.00, acc.held);
    }

    #[test]
    fn should_not_be_able_raise_a_dispute_if_funds_are_insufficient() {
        let mut service: TransactionService = Default::default();

        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.40),
        };

        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::WITHDRAWAL,
            client: 1,
            tx: 2,
            amount: Some(1.40),
        };

        let _ = service.process(record2);

        let record3 = TransactionRecord {
            _type: TransactionType::DISPUTE,
            client: 1,
            tx: 1,
            amount: None,
        };

        let result = service.process(record3);

        assert_eq!(Err(TransactionError::InsufficientFunds), result);
    }

    #[test]
    fn should_reverse_transaction_for_a_valid_chargeback() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };

        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::DISPUTE,
            client: 1,
            tx: 1,
            amount: None,
        };
        let _ = service.process(record2);

        let record3 = TransactionRecord {
            _type: TransactionType::CHARGEBACK,
            client: 1,
            tx: 1,
            amount: None,
        };
        let result = service.process(record3);

        assert_eq!(Ok(()), result);
        let acc = service.account_ledger.get(&1).unwrap();
        assert_eq!(0.00, acc.available);
        assert_eq!(0.00, acc.total);
        assert_eq!(0.00, acc.held);
        assert!(acc.locked);
    }

    #[test]
    fn should_not_resolve_a_transaction_which_is_not_in_dispute() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };
        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::RESOLVE,
            client: 1,
            tx: 1,
            amount: None,
        };
        let result = service.process(record2);

        assert_eq!(Err(TransactionError::TransactionNotDisputed), result);
    }

    #[test]
    fn should_not_process_transaction_for_a_locked_account() {
        let mut service: TransactionService = Default::default();
        let record1 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };
        let _ = service.process(record1);

        let record2 = TransactionRecord {
            _type: TransactionType::DISPUTE,
            client: 1,
            tx: 1,
            amount: None,
        };
        let _ = service.process(record2);

        let record2 = TransactionRecord {
            _type: TransactionType::CHARGEBACK,
            client: 1,
            tx: 1,
            amount: None,
        };
        let _ = service.process(record2);

        let record4 = TransactionRecord {
            _type: TransactionType::DEPOSIT,
            client: 1,
            tx: 1,
            amount: Some(1.50),
        };
        let result = service.process(record4);

        assert_eq!(Err(TransactionError::LockedAccount), result);
    }
}