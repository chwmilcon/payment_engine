#![allow(unused)]
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    io,
};

use crate::account::{AccountStatus, AccountStatusTotal};
use crate::transaction::{Transaction, TransactionType};
use csv::Writer;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct Ledger {
    pub by_client_id: HashMap<u16, AccountStatus>,
    pub by_transaction_id: HashMap<u32, Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            by_client_id: HashMap::new(),
            by_transaction_id: HashMap::new(),
        }
    }
    ///
    /// Is a given client id an existing client?
    ///
    /// Look in the ledger and see if the client id is a valid client already
    /// in the ledger
    ///
    /// # Arguments
    ///
    /// * `self`: Self
    /// * `client_id`: Id of client to check
    ///
    /// # Returns: True/False
    ///
    pub fn is_existing_client(&self, client_id: u16) -> bool {
        self.by_client_id.contains_key(&client_id)
    }
    ///
    /// Is a given transaction id already in the ledger?
    ///
    /// Look to see if a transaction id is already in the ledger
    ///
    /// # Arguments
    ///
    /// * `self`: Self
    /// * `trans_id`: Transaction Id
    ///
    /// # Returns True/False
    ///
    pub fn is_existing_transaction(&self, trans_id: u32) -> bool {
        self.by_transaction_id.contains_key(&trans_id)
    }
    ///
    /// dump ledger
    ///
    /// Dump the ledger, both clients and transaction.
    ///
    /// # Arguments
    ///
    /// * `self`: Self
    /// * `filename`: filename to write output to
    ///
    /// # Returns
    ///
    pub fn dump_ledger(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut temp_file = File::create(filename)?;
        serde_json::to_writer_pretty(temp_file, self)?;
        Ok(())
    }
    ///
    /// Write the clients to a `File`
    ///
    /// Write the clients to stdout or file as a csv, with titles.
    ///
    /// # Arguments
    ///
    /// * `self`: Self
    /// * `file`: File (either stdout or actual file)
    ///
    /// # Returns
    ///
    pub fn dump_client_csv<W: std::io::Write>(
        &self,
        wtr: &mut Writer<W>,
    ) -> Result<(), Box<dyn Error>> {
        for (_client_id, row) in &self.by_client_id {
            let row = AccountStatusTotal::new(row);
            wtr.serialize(row)?;
        }
        wtr.flush();
        Ok(())
    }
    ///
    /// Process a single transaction into the ledger
    ///
    /// A single transaction is passed into the function. It is processed into
    /// the ledger, creating the client if needed, verifying that the
    /// transaction id hasn't been used already.
    ///
    /// # Arguments
    ///
    /// * `self`: Self
    /// * `transaction`: Transaction
    ///
    /// # Returns
    /// `Result<(), Box<dyn Error>>`: Ok(()) if all transactions were processed successfully
    ///
    pub fn process_transaction(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Check to see if the client exists
        if !self.is_existing_client(transaction.client_id) {
            self.add_client(transaction.client_id);
        }
        // check to see if we've seen this transaction already
        if self.is_existing_transaction(transaction.tx_id) {
            return Err(format!("Transaction {} already seen", transaction.tx_id).into());
        }

        // Now process the actual transaction
        let result = match transaction.tx_type {
            TransactionType::Chargeback => self.process_chargeback(transaction),
            TransactionType::Deposit => self.process_deposit(transaction),
            TransactionType::Dispute => self.process_dispute(transaction),
            TransactionType::Resolve => self.process_resolve(transaction),
            TransactionType::Withdrawl => self.process_withdrawl(transaction),
        };
        result
    }
    ///
    /// add a client_id to the ledger, meaning we add an Account Status for this client.
    ///
    fn add_client(&mut self, client_id: u16) {
        let client_account = AccountStatus::new(client_id);
        self.by_client_id.insert(client_id, client_account);
    }
    //
    // Chargeback
    //
    // A chargeback is the final state of a dispute and represents the client
    // reversing a transaction. Funds that were held have now been
    // withdrawn. This means that the clients held funds and total
    // funds should decrease by the amount previously disputed. If a
    // chargeback occurs the client's account should be immediately
    // frozen.
    //
    fn process_chargeback(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Get the user account, it should be since we checked earlier.
        let account = self.by_client_id.get_mut(&transaction.client_id);
        if let Some(account) = account {
            // Look for old transaction
            let old_transaction = self.by_transaction_id.get(&transaction.tx_id);
            if let Some(old_transaction) = old_transaction {
                // TODO: Do I need to check to see if the new amount == old amount?
                if old_transaction.amount == transaction.amount {
                    return Err(format!(
                        "Old amount in transaction: {} was: {}, not equal to disputed amount {}",
                        transaction.tx_id, old_transaction.amount, transaction.amount
                    )
                    .into());
                } else {
                    // TODO: See if I'm dealing with amount, total and fields correctly.
                    // TODO: I don't think I need to deal with total since I've already done this.
                    //                    account.total += transaction.amount;
                    account.held -= transaction.amount;
                    // Note: How does this ever get unlocked?
                    account.locked = true;
                }
            } else {
                return Err(format!(
                    "Charge back transaction {} not found in ledger",
                    transaction.tx_id
                )
                .into());
            }
        } else {
            return Err(format!("Client {} not found in ledger", transaction.client_id).into());
        }
        Ok(())
    }
    //
    // Deposit
    //
    // A deposit is a credit to the client's asset account, meaning it
    // should increase the available and total funds of the client account
    //
    fn process_deposit(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Get the user account, it should be since we checked earlier.
        let account = self.by_client_id.get_mut(&transaction.client_id);
        if let Some(account) = account {
            account.available += transaction.amount;
        } else {
            return Err(format!("Client {} not found in ledger", transaction.client_id).into());
        }
        Ok(())
    }
    //
    // Dispute
    //
    // A dispute represents a client's claim that a transaction was
    // erroneous and should be reversed. The transaction shouldn't be
    // reversed yet but the associated funds should be held. This means
    // that the clients available funds should decrease by the amount
    // disputed, their held funds should increase by the amount disputed,
    // while their total funds should remain the same.
    //
    fn process_dispute(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Get the user account, it should be since we checked earlier.
        let account = self.by_client_id.get_mut(&transaction.client_id);
        if let Some(account) = account {
            // Look for old transaction
            let old_transaction = self.by_transaction_id.get(&transaction.tx_id);
            if let Some(old_transaction) = old_transaction {
                // TODO: Do I need to check to see if the new amount == old amount?
                if old_transaction.amount == transaction.amount {
                    return Err(format!(
                        "Old amount in transaction: {} was: {}, not equal to disputed amount {}",
                        transaction.tx_id, old_transaction.amount, transaction.amount
                    )
                    .into());
                } else {
                    // TODO: See if I'm dealing with amount, total and fields correctly.
                    account.available -= transaction.amount;
                    account.held += transaction.amount;
                }
            } else {
                return Err(format!(
                    "Disputed transaction {} not found in ledger",
                    transaction.tx_id
                )
                .into());
            }
        } else {
            return Err(format!("Client {} not found in ledger", transaction.client_id).into());
        }
        Ok(())
    }
    //
    // Resolve
    //
    // A resolve represents a resolution to a dispute, releasing the
    // associated held funds. Funds that were previously disputed are
    // no longer disputed. This means that the clients held funds
    // should decrease by the amount no longer disputed, their
    // available funds should increase by the amount no longer
    // disputed, and their total funds should remain the same.
    //
    fn process_resolve(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Get the user account, it should be since we checked earlier.
        let account = self.by_client_id.get_mut(&transaction.client_id);
        if let Some(account) = account {
            // Look for old transaction
            let old_transaction = self.by_transaction_id.get(&transaction.tx_id);
            if let Some(old_transaction) = old_transaction {
                // TODO: Do I need to check to see if the new amount == old amount?
                if old_transaction.amount == transaction.amount {
                    return Err(format!(
                        "Old amount in transaction: {} was: {}, not equal to disputed amount {}",
                        transaction.tx_id, old_transaction.amount, transaction.amount
                    )
                    .into());
                } else {
                    // TODO: See if I'm dealing with amount, total and fields correctly.
                    account.available += transaction.amount;
                    account.held -= transaction.amount;
                }
            } else {
                return Err(format!(
                    "Resolved transaction {} not found in ledger",
                    transaction.tx_id
                )
                .into());
            }
        } else {
            return Err(format!("Client {} not found in ledger", transaction.client_id).into());
        }
        Ok(())
    }
    //
    // Withdrawal
    //
    // A withdraw is a debit to the client's asset account, meaning it
    // should decrease the available and total funds of the client
    // account.
    //
    fn process_withdrawl(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Get the user account, it should be since we checked earlier.
        let account = self.by_client_id.get_mut(&transaction.client_id);
        if let Some(account) = account {
            // TODO: Do I need checks here to make sure it's not locked.
            if account.available >= transaction.amount {
                account.available -= transaction.amount;
            }
        } else {
            return Err(format!("Client {} not found in ledger", transaction.client_id).into());
        }

        Ok(())
    }
}
