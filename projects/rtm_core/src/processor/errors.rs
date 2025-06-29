use crate::models::{ClientId, TransactionId};

/// Represents possible errors during transaction processing.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum TransactionError {
    /// Current account is locked.
    AccountLocked { client_id: ClientId },

    /// Tried to apply Withdrawl with amount greater than available balance.
    InsufficientFunds { cause_id: TransactionId },

    /// Dispute, Resolve or Chargeback transaction refers to a non-existent transaction.
    TransactionDoesNotExist { ref_id: TransactionId },

    /// Tried to apply a transaction with id that was already prcoessed.
    DuplicateTransaction { cause_id: TransactionId },

    /// Tried to Dispute the same transaction twice.
    TransactionAlreadyDisputed { ref_id: TransactionId },

    /// Tried to Resolve or Chargeback a non-disputed transaction.
    TransactionNotDisputed { ref_id: TransactionId },

    /// Tried to apply a transaction to a different client.
    CrossClientTransaction,
}
