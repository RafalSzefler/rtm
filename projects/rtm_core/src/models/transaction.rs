use super::{Amount, ClientId, TransactionId};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
}

/// Represents an accounting operation that deals with the actual money.
#[derive(Debug)]
#[must_use]
pub struct Transaction {
    client_id: ClientId,
    id: TransactionId,
    amount: Amount,
    kind: TransactionKind,
}

impl Transaction {
    pub const fn new(client_id: ClientId, id: TransactionId, amount: Amount, kind: TransactionKind) -> Self {
        Self {
            client_id,
            id,
            amount,
            kind,
        }
    }

    pub const fn id(&self) -> TransactionId {
        self.id
    }

    pub const fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub const fn kind(&self) -> TransactionKind {
        self.kind
    }

    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
}
