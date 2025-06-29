use std::collections::HashMap;

use rstest::rstest;
use rtm_core::{
    models::{AccountingOperation, Amount, ClientId, Transaction, TransactionId, TransactionKind},
    processor::{AccountingSystem, ClientAccountState, TransactionError},
};

#[test]
fn test_accounting_system() {
    let mut accounting_system = AccountingSystem::new();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(1),
                Amount::from(100),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();

    let client_accounts = accounting_system.iter_accounts().collect::<Vec<_>>();
    assert_eq!(client_accounts.len(), 1);

    let first = client_accounts[0];
    assert_eq!(first.client_id, ClientId::from(1));
    assert_eq!(first.available_balance, Amount::from(100));
    assert_eq!(first.held_balance, Amount::zero());
    assert_eq!(first.state, ClientAccountState::Normal);
}

#[test]
fn test_accounting_system_2() {
    let mut accounting_system = AccountingSystem::new();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(1),
                Amount::from(1),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(2),
                TransactionId::from(2),
                Amount::from(2),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(3),
                Amount::from(2),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(4),
                Amount::try_from("1.5").unwrap(),
                TransactionKind::Withdrawal,
            ),
        })
        .unwrap();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(2),
                TransactionId::from(5),
                Amount::from(2),
                TransactionKind::Withdrawal,
            ),
        })
        .unwrap();

    let client_accounts = accounting_system
        .iter_accounts()
        .map(|c| (c.client_id, c))
        .collect::<HashMap<_, _>>();
    assert_eq!(client_accounts.len(), 2);

    let first = client_accounts.get(&ClientId::from(1)).unwrap();
    assert_eq!(first.client_id, ClientId::from(1));
    assert_eq!(first.available_balance, Amount::try_from("1.5").unwrap());
    assert_eq!(first.held_balance, Amount::zero());
    assert_eq!(first.state, ClientAccountState::Normal);

    let second = client_accounts.get(&ClientId::from(2)).unwrap();
    assert_eq!(second.client_id, ClientId::from(2));
    assert_eq!(second.available_balance, Amount::zero());
    assert_eq!(second.held_balance, Amount::zero());
    assert_eq!(second.state, ClientAccountState::Normal);
}

#[test]
fn test_accounting_system_too_big_withdrawal() {
    let mut accounting_system = AccountingSystem::new();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(1),
                Amount::from(1),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    let result = accounting_system.run_operation(AccountingOperation::Transaction {
        transaction: Transaction::new(
            ClientId::from(1),
            TransactionId::from(2),
            Amount::from(2),
            TransactionKind::Withdrawal,
        ),
    });
    assert!(matches!(result, Err(TransactionError::InsufficientFunds { .. })));
}

#[rstest]
#[case(AccountingOperation::Dispute { client_id: ClientId::from(1), ref_id: TransactionId::from(2) })]
#[case(AccountingOperation::Resolve { client_id: ClientId::from(1), ref_id: TransactionId::from(2) })]
#[case(AccountingOperation::Chargeback { client_id: ClientId::from(1), ref_id: TransactionId::from(2) })]
fn test_accounting_system_invalid_transaction(#[case] op: AccountingOperation) {
    let mut accounting_system = AccountingSystem::new();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(1),
                Amount::from(1),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    accounting_system
        .run_operation(AccountingOperation::Dispute {
            client_id: ClientId::from(1),
            ref_id: TransactionId::from(1),
        })
        .unwrap();
    let result = accounting_system.run_operation(op);
    assert!(matches!(result, Err(TransactionError::TransactionDoesNotExist { .. })));
}

#[rstest]
#[case(AccountingOperation::Resolve { client_id: ClientId::from(1), ref_id: TransactionId::from(1) })]
#[case(AccountingOperation::Chargeback { client_id: ClientId::from(1), ref_id: TransactionId::from(1) })]
fn test_accounting_system_not_dispuated(#[case] op: AccountingOperation) {
    let mut accounting_system = AccountingSystem::new();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(1),
                Amount::from(1),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    let result = accounting_system.run_operation(op);
    assert!(matches!(result, Err(TransactionError::TransactionNotDisputed { .. })));
}

#[rstest]
#[case(AccountingOperation::Dispute { client_id: ClientId::from(1), ref_id: TransactionId::from(1) })]
fn test_accounting_system_already_dispuated(#[case] op: AccountingOperation) {
    let mut accounting_system = AccountingSystem::new();
    accounting_system
        .run_operation(AccountingOperation::Transaction {
            transaction: Transaction::new(
                ClientId::from(1),
                TransactionId::from(1),
                Amount::from(1),
                TransactionKind::Deposit,
            ),
        })
        .unwrap();
    accounting_system
        .run_operation(AccountingOperation::Dispute {
            client_id: ClientId::from(1),
            ref_id: TransactionId::from(1),
        })
        .unwrap();
    let result = accounting_system.run_operation(op);
    assert!(matches!(
        result,
        Err(TransactionError::TransactionAlreadyDisputed { .. })
    ));
}
