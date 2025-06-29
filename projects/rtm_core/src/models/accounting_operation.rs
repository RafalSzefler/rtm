use crate::models::{ClientId, TransactionId};

use super::Transaction;

/// Represents an accounting operation that can be applied to the accounting system.
#[derive(Debug)]
#[must_use]
pub enum AccountingOperation {
    Transaction { transaction: Transaction },
    Dispute { client_id: ClientId, ref_id: TransactionId },
    Resolve { client_id: ClientId, ref_id: TransactionId },
    Chargeback { client_id: ClientId, ref_id: TransactionId },
}

impl AccountingOperation {
    pub const fn client_id(&self) -> ClientId {
        match self {
            AccountingOperation::Transaction { transaction } => transaction.client_id(),
            AccountingOperation::Dispute { client_id, .. }
            | AccountingOperation::Resolve { client_id, .. }
            | AccountingOperation::Chargeback { client_id, .. } => *client_id,
        }
    }
}
