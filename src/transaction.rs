#![allow(unused)] // TODO: remove after development

use core::fmt;
use csv::{Reader, Trim};
use log::{debug, error, info};
use rust_decimal::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TransactionType {
    Deposit,
    Withdrawl,
    Dispute,
    Resolve,
    Chargeback,
}
//
// Note: for this sample this is not necessary as we aren't threaded, but
//       in a production payment processor where transactions would be read
//       on multiple threads it would be.
static CURRENT_SEQ: AtomicU32 = AtomicU32::new(0);

// Define a struct to represent a transaction
#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub seq_num: u32, // seq_num differs from tx_id in that it's when we received it
    pub tx_type: TransactionType,
    pub client_id: u16,
    pub tx_id: u32, // Transaction id as established by something outside of this
    pub amount: rust_decimal::Decimal,
}

impl Transaction {
    /// Output a single transaction
    ///
    /// Write a single transaction as a json blob to an open file. This is
    /// primarily for debugging.
    ///
    /// # Arguments
    ///
    /// * `self`: Self
    /// * `file`: file to to write json blog to. Should be open and ready to write
    ///
    /// # Returns: Ok, Error
    pub fn output(&self, file: &mut File) -> Result<(), Box<dyn Error>> {
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: type:{}", self.seq_num, self.amount)
    }
}

/// Reads a CSV file from the given filename and processes each row using a provided function.
///
/// This function handles opening the file and creating the CSV reader, then delegates
/// to `process_csv_from_reader` for the actual processing.
///
/// # Arguments
///
/// * `filename`: The path to the CSV file.
/// * `process_func`: A closure that takes a `Transaction` and returns a `Result<(), Box<dyn Error>>`.
/// * `keep_going` : Keep going if you have an erroneous line true/false
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`: Ok(()) if all transactions were processed successfully,
///   otherwise an error indicating the first encountered issue.
pub fn process_file<F>(
    filename: &str,
    process_func: F,
    keep_going: bool,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(Transaction) -> Result<(), Box<dyn Error>>,
{
    let file = File::open(filename)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .from_reader(file);

    process_csv_from_reader(rdr, process_func, keep_going)
}

/// Translate that we have a transaction type from string to enum TransactionType
///
/// # Arguments
///
/// * `transaction_type`: Transaction type to check.
///
/// # Returns
///
/// * Result<TransactionType, Box<dyn Error>>
pub fn translate_trx_type(trx_type: &str) -> Result<TransactionType, Box<dyn Error>> {
    let result = match trx_type {
        "withdrawal" => Ok(TransactionType::Withdrawl),
        "withdraw" => Ok(TransactionType::Withdrawl),
        "deposit" => Ok(TransactionType::Deposit),
        "dispute" => Ok(TransactionType::Dispute),
        "resolve" => Ok(TransactionType::Resolve),
        "chargeback" => Ok(TransactionType::Chargeback),
        _ => Err(format!("Unknown Tranaction {}", trx_type).into()),
    };
    result
}

/// Reads a CSV from a string buffer and processes each row using a provided function.
///
/// # Arguments
///
/// * `buffer`: The string buffer containing CSV data.
/// * `process_func`: A closure that takes a `Transaction` and returns a `Result<(), Box<dyn Error>>`.
/// * `keep_going` : Keep going if you have an erroneous line true/false
///
/// # Returns
///
/// * Result<TransactionType, Box<dyn Error>>
pub fn process_csv_from_buffer<F>(
    buffer: &str,
    process_func: F,
    keep_going: bool,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(Transaction) -> Result<(), Box<dyn Error>>,
{
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(buffer.as_bytes());

    process_csv_from_reader(rdr, process_func, keep_going)
}

/// Reads a CSV file and processes each row using a provided function.
///
/// # Arguments
///
/// * `rdr`: A `csv::Reader` instance from which to read records.
/// * `process_func`: A closure that takes a `Transaction` and returns a `Result<(), Box<dyn Error>>`.
/// * `keep_going` : Keep going if we have an erroneous line. true/false
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`: Ok(()) if all transactions were processed successfully,
///   otherwise an error indicating the first encountered issue.
///
pub fn process_csv_from_reader<R: Read, F>(
    mut rdr: Reader<R>,
    mut process_func: F,
    keep_going: bool,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(Transaction) -> Result<(), Box<dyn Error>>,
{
    let mut cnt: u32 = 0;
    for result in rdr.records() {
        cnt + 1;
        let row_result = match result {
            Ok(result) => {
                let row_result = process_row(result, cnt);
                let result = match row_result {
                    Ok(row) => process_func(row),
                    Err(e) => Err(e),
                };
                result
            }
            Err(ref err) => Err(err.into()),
        };
        if let Err(err) = row_result {
            if keep_going {
                error!("Error: {}", err);
                continue;
            } else {
                return Err(format!("process_func: {}", err).into());
            }
        }
    }

    Ok(())
}

//
// process_row - process a single row. Broken out from the above function so that
//               there is finer grain control over continue/stop functionality.
//
fn process_row(record: csv::StringRecord, cnt: u32) -> Result<Transaction, Box<dyn Error>> {
    // Ensure the record has the expected number of fields
    if record.len() != 4 {
        return Err(format!(
            "Invalid record format: expected 4 fields, got {}. Line: {}",
            record.len(),
            cnt
        )
        .into());
    }

    let tx_type = record.get(0).ok_or("Missing type field")?.to_string();
    let tx_type = translate_trx_type(&tx_type)?;
    let client_id_str = record.get(1).ok_or("Missing client field")?;
    let tx_id_str = record.get(2).ok_or("Missing tx field")?;
    let amount_str = record.get(3).ok_or("Missing amount field")?;

    // Parse client_id
    let client_id = client_id_str
        .parse::<u16>()
        .map_err(|e| format!("Failed to parse client ID '{}': {}", client_id_str, e))?;

    // Parse tx_id
    let tx_id = tx_id_str
        .parse::<u32>()
        .map_err(|e| format!("Failed to parse transaction ID '{}': {}", tx_id_str, e))?;

    // Parse amount using rust_decimal for precise decimal handling
    let amount = rust_decimal::Decimal::from_str(amount_str)
        .map_err(|e| format!("Failed to parse amount '{}': {}", amount_str, e))?;

    // Amount's should be at most 4 places
    let rounded_amount = amount.round_dp(4);

    if amount != rounded_amount {
        return Err(format!("Amount not formatted correctly {}", amount_str).into());
    }

    // Note: atomic here is a bit of an overkill in this example, but
    // would be needed in production/multithreaded version.
    let seq_num = CURRENT_SEQ.fetch_add(1, Ordering::Relaxed);
    let transaction = Transaction {
        seq_num,
        tx_type,
        client_id,
        tx_id,
        amount,
    };
    Ok(transaction)
}
//////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper function to create a temporary CSV file
    fn create_temp_csv(content: &str) -> Result<NamedTempFile, Box<dyn Error>> {
        let mut file = NamedTempFile::new()?;
        write!(file, "{}", content)?;
        file.flush()?;
        Ok(file)
    }

    #[test]
    fn test_process_success() -> Result<(), Box<dyn Error>> {
        let csv_content =
            "type, client, tx, amount\ndeposit,101,1000001,123.4567\nwithdraw,202,1000002,78.90\ndeposit,101,1000003,50.00";
        let mut processed_transactions = Vec::new();
        let process_func = |tx: Transaction| -> Result<(), Box<dyn Error>> {
            processed_transactions.push(tx.clone());
            Ok(())
        };

        process_csv_from_buffer(csv_content, process_func, false)?;

        assert_eq!(processed_transactions.len(), 3);
        assert_eq!(processed_transactions[0].tx_type, TransactionType::Deposit);
        assert_eq!(processed_transactions[0].client_id, 101);
        assert_eq!(processed_transactions[0].tx_id, 1000001);
        assert_eq!(
            processed_transactions[0].amount,
            rust_decimal::Decimal::from_str("123.4567")?
        );

        assert_eq!(
            processed_transactions[1].tx_type,
            TransactionType::Withdrawl
        );
        assert_eq!(processed_transactions[1].client_id, 202);
        assert_eq!(processed_transactions[1].tx_id, 1000002);
        assert_eq!(
            processed_transactions[1].amount,
            rust_decimal::Decimal::from_str("78.90")?
        );

        assert_eq!(processed_transactions[2].tx_type, TransactionType::Deposit);
        assert_eq!(processed_transactions[2].client_id, 101);
        assert_eq!(processed_transactions[2].tx_id, 1000003);
        assert_eq!(
            processed_transactions[2].amount,
            rust_decimal::Decimal::from_str("50.00")?
        );

        Ok(())
    }

    #[test]
    fn test_process_file_file_not_found() {
        let result = process_file("non_existent_file.csv", |_| Ok(()), false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No such file or directory"));
    }

    #[test]
    fn test_process_invalid_client_id() -> Result<(), Box<dyn Error>> {
        let csv_content = "type, client, tx, amount\ndeposit,abc,1000001,100.00";
        let result = process_csv_from_buffer(csv_content, |_| Ok(()), false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse client ID 'abc'"));
        Ok(())
    }

    #[test]
    fn test_amount_precesion() -> Result<(), Box<dyn Error>> {
        // Note: Not testing valid number of digits specifically as that is tested in other test cases
        let csv_content = "type, client, tx, amount\ndeposit,101,1000001,123.45678";
        let result = process_csv_from_buffer(csv_content, |_| Ok(()), false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Amount not formatted correctly 123.45678"));

        let csv_content = "type, client, tx, amount\ndeposit,101,1000001,123.0";
        process_csv_from_buffer(csv_content, |_| Ok(()), false)?;

        let csv_content = "type, client, tx, amount\ndeposit,101,1000001,123";
        process_csv_from_buffer(csv_content, |_| Ok(()), false)?;

        Ok(())
    }

    #[test]
    fn test_process_invalid_tx_id() -> Result<(), Box<dyn Error>> {
        let csv_content = "type, client, tx, amount\ndeposit,101,xyz,100.00";
        let result = process_csv_from_buffer(csv_content, |_| Ok(()), false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse transaction ID 'xyz'"));
        Ok(())
    }

    #[test]
    fn test_process_invalid_amount() -> Result<(), Box<dyn Error>> {
        let csv_content = "type, client, tx, amount\ndeposit,101,1000001,not_a_number";
        let result = process_csv_from_buffer(csv_content, |_| Ok(()), false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse amount 'not_a_number'"));
        Ok(())
    }

    #[test]
    fn test_process_invalid_record_format() -> Result<(), Box<dyn Error>> {
        let csv_content = "type, client, tx, amount\ndeposit,101,1000001"; // Missing amount field
        let result = process_csv_from_buffer(csv_content, |_| Ok(()), false);
        assert!(result.is_err());
        // Note: contains() isn't very resilient, should/would improve
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("CSV error: record 1 (line: 2, byte: 25): found record with 3 fields, but the previous record has 4 fields"));
        Ok(())
    }

    #[test]
    fn test_process_file_processing_function_error() -> Result<(), Box<dyn Error>> {
        let csv_content = "type, client, tx, amount\ndeposit,101,1000001,100.00";
        let temp_file = create_temp_csv(csv_content)?;
        let filename = temp_file
            .path()
            .to_str()
            .ok_or("Failed to get temp file path")?;

        let mut call_count = 0;
        let process_func = |_tx: Transaction| -> Result<(), Box<dyn Error>> {
            call_count += 1;
            Err("Error during processing".into())
        };

        let result = process_file(filename, process_func, false);
        assert!(result.is_err());
        let str = result.unwrap_err().to_string();
        assert_eq!(str, "process_func: Error during processing");
        assert_eq!(call_count, 1); // Should only call the function once before erroring
        Ok(())
    }

    #[test]
    fn test_process_csv_from_buffer_success() -> Result<(), Box<dyn Error>> {
        // TODO: Is this redundant with another test?
        let csv_content =
            "type, client, tx, amount\ndeposit,101,1000001,123.4567\nwithdraw,202,1000002,78.90";

        let mut processed_transactions = Vec::new();
        let process_func = |tx: Transaction| -> Result<(), Box<dyn Error>> {
            processed_transactions.push(tx);
            Ok(())
        };

        process_csv_from_buffer(csv_content, process_func, false)?;

        assert_eq!(processed_transactions.len(), 2);
        assert_eq!(processed_transactions[0].tx_type, TransactionType::Deposit);
        assert_eq!(processed_transactions[0].client_id, 101);
        assert_eq!(processed_transactions[0].tx_id, 1000001);
        assert_eq!(
            processed_transactions[0].amount,
            rust_decimal::Decimal::from_str("123.4567")?
        );
        assert_eq!(
            processed_transactions[1].amount,
            rust_decimal::Decimal::from_str("78.90")?
        );

        Ok(())
    }

    #[test]
    // TODO: Remove, just for debugging
    fn foo_dump_transaction() -> Result<(), Box<dyn Error>> {
        let transaction = Transaction {
            seq_num: 1,
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amount: rust_decimal::Decimal::from_str("100.00")?,
        };

        //let mut temp_file = NamedTempFile::new()?;
        let mut temp_file = File::create("output.json")?;

        transaction.output(&mut temp_file)?;

        //         let mut buffer = String::new();
        //         temp_file.as_file_mut().read_to_string(&mut buffer)?;

        //         let expected_output = r#"{
        //   "seq_num": 1,
        //   "tx_type": "Deposit",
        //   "client_id": 1,
        //   "tx_id": 1,
        //   "amount": 100.00
        // }"#;
        //         assert_eq!(buffer.trim(), expected_output.trim());

        Ok(())
    }
}
