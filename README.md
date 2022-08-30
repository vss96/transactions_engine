# transactions_engine

A transaction system which takes in an input of different transactions for different clients
and keeps track of the Accounts involved in the transaction. After processing all transactions,
it prints the various account balances for each account to STD.


### Different types of transactions:
* Deposit: Increases the available and total amount in the account. Does not involve any error scenarios.
* Withdrawal: Decreases the available and total amount in the account. If the withdrawal amount is greater than what's available it errors out.
* Dispute: Creates a dispute for existing deposits and withdrawals. The amount disputed is held and removed from your available balance. 
  If the transaction is not a deposit or withdrawal, it is ignored (we only keep track of those transactions in the first place).
* Resolve: Dispute no longer exists and held amount is transferred back to the available balance.
* Chargeback: Disputed transaction is reversed and the account is locked.

#### Notes:
* Resolve and chargeback are very similar other than how they change the values in the Account itself.
* Once a chargeback occurs for a valid dispute, the account is locked and can't undergo any further transactions.
* When a dispute is raised and if the amount disputed is greater than whatever balance is available, the dispute is ignored.
* If a dispute is resolved, the transaction that was previously under dispute can be disputed again.
* You shouldn't be able to dispute transactions that belong to a different client, which made me change my implementation from storing
amount against the tx id to storing both amount and the client id.


### How to run:

``cargo run -- input.csv > output.csv``

### How to enable logging:

`` export RUST_LOG=transactions_engine=info``