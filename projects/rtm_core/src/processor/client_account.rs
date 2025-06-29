use std::collections::{HashMap, HashSet};

use crate::models::{Amount, ClientId, Transaction, TransactionId};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[must_use]
pub enum ClientAccountState {
    #[default]
    Normal,
    Locked,
}

#[derive(Debug)]
#[must_use]
pub struct ClientAccount {
    pub client_id: ClientId,
    pub available_balance: Amount,
    pub held_balance: Amount,
    pub state: ClientAccountState,
    pub(super) transactions: HashMap<TransactionId, Transaction>,
    pub(super) disputed_transactions: HashSet<TransactionId>,
}

impl ClientAccount {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            available_balance: Amount::default(),
            held_balance: Amount::default(),
            state: ClientAccountState::default(),
            transactions: HashMap::default(),
            disputed_transactions: HashSet::default(),
        }
    }
}
