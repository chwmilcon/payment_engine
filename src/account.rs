#![allow(dead_code)] // CHW: Remove after development
#![allow(unused)] use rust_decimal::Decimal;
use serde::Serialize;

//
// AccountStatus - everything but the total, we'll calcuate that
// during serialization for output.
//
#[derive(Debug, Serialize)]
pub struct AccountStatus {
    pub client: u16,
    #[serde(with = "rust_decimal::serde::str")]
    pub available: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub held: rust_decimal::Decimal,
    pub locked: bool,
}

#[derive(Debug, Serialize)]
pub struct AccountStatusTotal {
    pub client: u16,
    #[serde(with = "rust_decimal::serde::str")]
    pub available: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub held: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total: rust_decimal::Decimal,
    pub locked: bool,
}

impl AccountStatus {
    pub fn new(id : u16) -> Self {
        AccountStatus {
            client : id,
            available : Decimal::ZERO,
            held : Decimal::ZERO,
            locked : false,
        }
    }
}

impl AccountStatusTotal {
    pub fn new(source: &AccountStatus) -> Self {
        AccountStatusTotal {
            client : source.client,
            available : source.available,
            held : source.held,
            locked : source.locked,
            total : source.available + source.held
        }
    }
}
