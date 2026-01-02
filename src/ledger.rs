#![allow(unused)]
use std::{collections::{
    HashMap, hash_map::Entry::{Occupied, Vacant}
}, io};

use crate::{account::AccountStatus, transaction::TransactionType};
use crate::transaction::Transaction;
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

    // TODO: Clean up
    //pub fn dump_client_csv(&self, file: &mut File) -> Result<(), Box<dyn Error>> {
    pub fn dump_client_csv<W: std::io::Write>(&self, wtr: &mut Writer<W>) -> Result<(), Box<dyn Error>> {
        //let mut wtr = Writer::from_writer(file);
        //let mut wtr = Writer::from_writer(io::stdout());
        for (_client_id, row) in &self.by_client_id {
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
    pub fn process_transaction(&mut self, transaction : &Transaction) -> Result<(), Box<dyn Error>> {
        // Check to see if the client exists
        if ! self.is_existing_client(transaction.client_id) {
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

    // ////////////////////////////////////////////////////////////////////
    // TODO
    // ///////////////////////////////////////////////////////////////////
    fn process_chargeback(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn process_deposit(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn process_dispute(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn process_resolve(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn process_withdrawl(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
