use std::collections::{HashMap, HashSet};

use crate::{
    models::{AccountingOperation, Amount, ClientId, Transaction, TransactionId, TransactionKind},
    processor::ClientAccountState,
};

use super::{ClientAccount, TransactionError};

#[derive(Debug)]
#[must_use]
pub struct AccountingSystem {
    client_accounts: HashMap<ClientId, ClientAccount>,
    seen_transactions: HashSet<TransactionId>,
}

impl AccountingSystem {
    pub fn new() -> Self {
        Self {
            client_accounts: HashMap::new(),
            seen_transactions: HashSet::new(),
        }
    }

    /// Runs a transaction over the existing accounting system state.
    ///
    /// # Errors
    ///
    /// For especific errors see [`TransactionError`].
    #[allow(clippy::missing_panics_doc)]
    pub fn run_operation(&mut self, operation: AccountingOperation) -> Result<(), TransactionError> {
        let client_id = operation.client_id();

        let client_account = self
            .client_accounts
            .entry(client_id)
            .or_insert_with(|| ClientAccount::new(client_id));

        if client_account.state == ClientAccountState::Locked {
            return Err(TransactionError::AccountLocked { client_id: client_id });
        }

        macro_rules! referred_transaction {
            ($transaction_id: expr) => {{
                let transaction_id = ($transaction_id).clone();
                if let Some(tr) = client_account.transactions.get(&transaction_id) {
                    tr
                } else {
                    return Err(TransactionError::TransactionDoesNotExist {
                        ref_id: transaction_id,
                    });
                }
            }};
        }

        match operation {
            AccountingOperation::Transaction { transaction } => {
                let amount = normalize_amount(&transaction);
                let new_amount = client_account.available_balance.clone() + amount;
                if new_amount < Amount::zero() {
                    return Err(TransactionError::InsufficientFunds {
                        cause_id: transaction.id(),
                    });
                }

                let transaction_id = transaction.id();
                if self.seen_transactions.contains(&transaction_id) {
                    return Err(TransactionError::DuplicateTransaction {
                        cause_id: transaction_id,
                    });
                }
                self.seen_transactions.insert(transaction_id);

                client_account.available_balance = new_amount;
                client_account.transactions.insert(transaction_id, transaction);
            }
            AccountingOperation::Dispute {
                client_id,
                ref_id: transaction_id,
            } => {
                let referred_transaction = referred_transaction!(transaction_id);

                if client_id != referred_transaction.client_id() {
                    return Err(TransactionError::CrossClientTransaction);
                }

                if !client_account.disputed_transactions.insert(transaction_id) {
                    return Err(TransactionError::TransactionAlreadyDisputed { ref_id: transaction_id });
                }

                let amount = normalize_amount(referred_transaction);

                client_account.held_balance += amount.clone();
                client_account.available_balance -= amount;
            }
            AccountingOperation::Resolve {
                client_id,
                ref_id: transaction_id,
            } => {
                let referred_transaction = referred_transaction!(transaction_id);
                if client_id != referred_transaction.client_id() {
                    return Err(TransactionError::CrossClientTransaction);
                }

                if !client_account.disputed_transactions.remove(&transaction_id) {
                    return Err(TransactionError::TransactionNotDisputed { ref_id: transaction_id });
                }

                let amount = normalize_amount(referred_transaction);

                client_account.held_balance -= amount.clone();
                client_account.available_balance += amount;
            }
            AccountingOperation::Chargeback {
                client_id,
                ref_id: transaction_id,
            } => {
                let referred_transaction = referred_transaction!(transaction_id);
                if client_id != referred_transaction.client_id() {
                    return Err(TransactionError::CrossClientTransaction);
                }

                if !client_account.disputed_transactions.remove(&transaction_id) {
                    return Err(TransactionError::TransactionNotDisputed { ref_id: transaction_id });
                }

                let amount = normalize_amount(referred_transaction);

                client_account.held_balance -= amount.clone();
                client_account.state = ClientAccountState::Locked;
            }
        }
        Ok(())
    }

    /// Iterates over all currently tracked client accounts.
    pub fn iter_accounts(&self) -> impl Iterator<Item = &ClientAccount> {
        self.client_accounts.values()
    }
}

impl Default for AccountingSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_amount(referred_transaction: &Transaction) -> Amount {
    let amount = referred_transaction.amount().clone();
    match referred_transaction.kind() {
        TransactionKind::Deposit => amount,
        TransactionKind::Withdrawal => -amount,
    }
}
