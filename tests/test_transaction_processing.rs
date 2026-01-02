#![allow(unused)]
//
// Transaction processing and the impact on the Ledger
//

// # Tests:
use payment_engine::{
    account::{AccountStatus, AccountStatusTotal},
    ledger::Ledger,
    transaction::{Transaction, TransactionType},
};
use rust_decimal::Decimal;
use std::error::Error;

// Helper function to create a transaction
fn create_transaction(
    tx_type: TransactionType,
    client_id: u16,
    tx_id: u32,
    amount: &str,
) -> Transaction {
    Transaction {
        seq_num: 0, // Not relevant for these tests
        tx_type,
        client_id,
        tx_id,
        amount: Decimal::from_str_exact(amount).unwrap(),
    }
}

#[test]
//
// * Make a deposit for client 1 of $200
// * Check to see if client 1 has (available=200, held = 0, locked=false)
//
fn test_deposit_single_client_single_deposit() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Test 1: Make a deposit for client 1 of $200
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "200.00");
    ledger.process_transaction(&tx1)?;

    // Check to see if client 1 has total of $200 and available for $200
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("200.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("200.00")?
    );
    assert_eq!(client1_status.locked, false);

    Ok(())
}

//
// * Make a deposit for client 1 of $200
// * Make a second deposit for client 1 of $200
// * Check to see if client 1 has (available=$400, held = 0, locked=false)
//
#[test]
fn test_deposit_two_deposits_single_client() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $200
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "200.00");
    ledger.process_transaction(&tx1)?;

    // Make second deposit for client 1 of $200
    let tx2 = create_transaction(TransactionType::Deposit, 1, 2, "200.00");
    ledger.process_transaction(&tx2)?;

    // Check to see if client 1 has (available=400, held = 0, locked=false)
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("400.00")?
    );
    assert_eq!(client1_status.locked, false);

    Ok(())
}
//
// * Make a deposit for client 1 of $200
// * Make second deposit for client 2 of $200
// * Check to see if client 1 has (available=200, held = 0, locked=false)
// * Check to see if client 2 has (available=200, held = 0, locked=false)
//
#[test]
fn test_one_deposit_two_clients() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $200
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "200.00");
    ledger.process_transaction(&tx1)?;

    // Make second deposit for client 2 of $200
    let tx2 = create_transaction(TransactionType::Deposit, 2, 2, "200.00");
    ledger.process_transaction(&tx2)?;

    // Check to see if client 1 has (available=200, held = 0, locked=false)
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("200.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(client1_status.locked, false);

    // Check to see if client 2 has (available=200, held = 0, locked=false)
    let client2_status = ledger.by_client_id.get(&2).unwrap();
    assert_eq!(client2_status.available, Decimal::from_str_exact("200.00")?);
    assert_eq!(client2_status.held, Decimal::ZERO);
    assert_eq!(client2_status.locked, false);

    Ok(())
}
//
// * Make a deposit for client 1 of $200
// * Make a withdrawl for client 1 of $100
// * Check to see if client 1 has (available=100, held = 0, locked=false)
//
#[test]
fn test_one_deposit_one_withdrawal_one_client() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $200
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "200.00");
    ledger.process_transaction(&tx1)?;

    // Make a withdrawal for client 1 of $100
    let tx2 = create_transaction(TransactionType::Withdrawl, 1, 2, "100.00");
    ledger.process_transaction(&tx2)?;

    // Check to see if client 1 has (available=100, held = 0, locked=false)
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("100.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("100.00")?
    );
    assert_eq!(client1_status.locked, false);

    Ok(())
}

//
// * Make a deposit for client 1 of $200 tx_id = 1
// * Make a withdrawl for client 1 of $100 tx_id = 2
// * Make a dispute for client 1 tx_id = 2 amount = $100
// * Check to see if client 1 has (available=100, held = 100, locked=false)
//
#[test]
fn test_one_deposit_one_withdrawl_one_dispute_one_client() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $200 tx_id = 1
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "200.00");
    ledger.process_transaction(&tx1)?;

    // Make a withdrawl for client 1 of $100 tx_id = 2
    let tx2 = create_transaction(TransactionType::Withdrawl, 1, 2, "100.00");
    ledger.process_transaction(&tx2)?;

    // Make a dispute for client 1 tx_id = 2 amount = $100
    // Note: The dispute transaction's amount should match the original transaction's amount.
    // The current implementation of process_dispute checks this.
    let tx3 = create_transaction(TransactionType::Dispute, 1, 2, "100.00");
    ledger.process_transaction(&tx3)?;

    // Check to see if client 1 has (available=0, held = 100, locked=false)
    // After withdrawal: available = 100.00
    // After dispute of tx_id 2 (amount 100.00): available -= 100.00, held += 100.00
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("0.00")?);
    assert_eq!(client1_status.held, Decimal::from_str_exact("100.00")?);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("100.00")?
    );
    assert_eq!(client1_status.locked, false);
    Ok(())
}

//
// * Make a deposit for client 1 of $400 tx_id = 1
// * Make a dispute for client 1 for $400 tx_id = 1
// * Check to see if client 1 has (available=200, held = 200, locked=false)
// * Make a resolve for client 1 for $400 tx_id = 1
// * Check to see if client 1 has (available=400, held = 0, locked=false)
//
#[test]
fn test_deposit_dispute_resolve_one_client() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $400 tx_id = 1
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "400.00");
    ledger.process_transaction(&tx1)?;

    // Check initial state
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("400.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make a dispute for client 1 for $400 tx_id = 1
    let tx2 = create_transaction(TransactionType::Dispute, 1, 1, "400.00");
    ledger.process_transaction(&tx2)?;

    // Check state after dispute
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("0.00")?);
    assert_eq!(client1_status.held, Decimal::from_str_exact("400.00")?);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("400.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make a resolve for client 1 for $400 tx_id = 1
    let tx3 = create_transaction(TransactionType::Resolve, 1, 1, "400.00");
    ledger.process_transaction(&tx3)?;

    // Check state after resolve
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(client1_status.locked, false);
    Ok(())
}

//
// * Make a deposit for client 1 of $400 tx_id = 1
// * Make a dispute for client 1 for $400 tx_id = 1
// * Check to see if client 1 has (available=200, held = 200, locked=false)
// * Make a deposit for client 1 of $400 tx_id = 1
// * Check to see if client 1 has (available=600, held = 200, locked=false)
// * Make a resolve for client 1 for $400 tx_id = 1
// * Check to see if client 1 has (available=800, held = 0, locked=false)
//
#[test]
fn test_deposit_dispute_deposit_resolve_one_client() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $400 tx_id = 1
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "400.00");
    ledger.process_transaction(&tx1)?;

    // Check initial state
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("400.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make a dispute for client 1 for $400 tx_id = 1
    let tx2 = create_transaction(TransactionType::Dispute, 1, 1, "400.00");
    ledger.process_transaction(&tx2)?;

    // Check state after dispute
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("0.00")?);
    assert_eq!(client1_status.held, Decimal::from_str_exact("400.00")?);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("400.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make another deposit for client 1 of $400 tx_id = 2 (new transaction)
    let tx3 = create_transaction(TransactionType::Deposit, 1, 2, "400.00");
    ledger.process_transaction(&tx3)?;

    // Check state after second deposit
    // available: 0.00 (from dispute) + 400.00 (new deposit) = 400.00
    // held: 400.00
    // total: 400.00 + 400.00 = 800.00
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::from_str_exact("400.00")?);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("800.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make a resolve for client 1 for $400 tx_id = 1
    let tx4 = create_transaction(TransactionType::Resolve, 1, 1, "400.00");
    ledger.process_transaction(&tx4)?;

    // Check state after resolve
    // available: 400.00 (from previous) + 400.00 (from resolve) = 800.00
    // held: 400.00 (from previous) - 400.00 (from resolve) = 0.00
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("800.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(client1_status.locked, false);
    Ok(())
}

//
// * Make a deposit for client 1 of $400 tx_id = 1
// * Make a deposit for client 1 of $400 tx_id = 2
// * Check to see if client 1 has (available=800, held = , locked=false)
// * Make a chargeback for client 1 for $400 tx_id = 2
// * Check to see if client 1 has (available=400, held = 0, locked=true)
//
#[test]
fn test_deposit_deposit_chargeback_one_client() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::new();

    // Make a deposit for client 1 of $400 tx_id = 1
    let tx1 = create_transaction(TransactionType::Deposit, 1, 1, "400.00");
    ledger.process_transaction(&tx1)?;

    // Make a deposit for client 1 of $400 tx_id = 2
    let tx2 = create_transaction(TransactionType::Deposit, 1, 2, "400.00");
    ledger.process_transaction(&tx2)?;

    // Check initial state
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("800.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("800.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make a dispute for client 1 for $400 tx_id = 2
    let tx3 = create_transaction(TransactionType::Dispute, 1, 2, "400.00");
    ledger.process_transaction(&tx3)?;

    // Check state after dispute
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("800.00")?
    );
    assert_eq!(client1_status.locked, false);

    // Make a chargeback for client 1 for $400 tx_id = 2
    let tx4 = create_transaction(TransactionType::Chargeback, 1, 2, "400.00");
    ledger.process_transaction(&tx4)?;

    // Check state after chargeback
    let client1_status = ledger.by_client_id.get(&1).unwrap();
    assert_eq!(client1_status.available, Decimal::from_str_exact("400.00")?);
    assert_eq!(client1_status.held, Decimal::ZERO);
    assert_eq!(
        AccountStatusTotal::new(client1_status).total,
        Decimal::from_str_exact("400.00")?
    );
    assert_eq!(client1_status.locked, true);
    Ok(())
}
