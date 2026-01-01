#![allow(dead_code)] // CHW: Remove after development
#![allow(unused)] // CHW: Remove after development
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
