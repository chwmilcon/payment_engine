#![allow(unused)]
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use crate::account::AccountStatus;
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
    pub fn is_existing_client(&self, client_id: u16) -> bool {
        self.by_client_id.contains_key(&client_id)
    }

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
    pub fn is_existing_transaction(&self, trans_id: u32) -> bool {
        self.by_transaction_id.contains_key(&trans_id)
    }

    pub fn dump_ledger(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut temp_file = File::create("output.json")?;
        serde_json::to_writer_pretty(temp_file, self)?;
        Ok(())
    }

    pub fn dump_client_csv(&self, file: &mut File) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_writer(file);
        for (_client_id, row) in &self.by_client_id {
            wtr.serialize(row)?;
        }
        wtr.flush();
        Ok(())
    }
}

#[test]
// TODO: Just for debugging, remove
pub fn test_dump_ledger() -> Result<(), Box<dyn Error>> {
    let ledger = Ledger::new();
    ledger.dump_ledger("output.json");
    Ok(())
}

#[test]
// TODO: Just for debugging, remove
pub fn test_dump_client_csv() -> Result<(), Box<dyn Error>> {
    // create Ledger
    // Add several clients to it
    // open file output.json
    // call ledger.dump_client_csv( with open file)?)?;
    let mut ledger = Ledger::new();
    ledger.by_client_id.insert(
        1,
        AccountStatus {
            client: 1,
            available: rust_decimal::Decimal::from_str("100.00")?,
            held: rust_decimal::Decimal::from_str("0.00")?,
            total: rust_decimal::Decimal::from_str("100.00")?,
            locked: false,
        },
    );
    ledger.by_client_id.insert(
        2,
        AccountStatus {
            client: 2,
            available: rust_decimal::Decimal::from_str("200.00")?,
            held: rust_decimal::Decimal::from_str("50.00")?,
            total: rust_decimal::Decimal::from_str("250.00")?,
            locked: true,
        },
    );

    let mut temp_file = File::create("clients.csv")?;
    ledger.dump_client_csv(&mut temp_file)?;

    Ok(())
}
