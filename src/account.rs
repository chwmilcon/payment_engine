#![allow(dead_code)] // CHW: Remove after development
#![allow(unused)] use rust_decimal::Decimal;
// CHW: Remove after development
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountStatus {
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
            total : Decimal::ZERO,
            locked : false,
        }
    }

}