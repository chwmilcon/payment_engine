#![allow(unused)] // TODO: remove after development
use clap::Parser;
use csv::Writer;
use env_logger::Builder;
use log::{debug, error, info};
use std::io::{self, Write};
use std::process::Stdio;

mod account;
mod args;
mod ledger;
mod transaction;
use crate::args::Args;
use crate::ledger::Ledger;
use crate::transaction::Transaction;

fn main() {
    let args = Args::parse();

    let mut builder = Builder::new();

    // Set up the logger at the debug level if the debug flag is present
    if args.debug {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }

    if let Some(ref logfile) = args.logfile {
        let target = Box::new(std::fs::File::create(logfile).expect("Can't create log file"));
        builder.target(env_logger::Target::Pipe(target));
    }

    builder.init();

    debug!("processing");

    let mut ledger = Ledger::new();

    let process_func = |transaction: Transaction| ledger.process_transaction(&transaction);

    if let Err(e) = transaction::process_file(
        args.name.to_str().unwrap(),
        process_func,
        !args.stop_on_error,
    ) {
        error!("Error processing CSV: {}", e);
        std::process::exit(1);
    }

    // since we processed everything given to us, output the client list
    let mut wtr = Writer::from_writer(io::stdout());
    ledger.dump_client_csv(&mut wtr);
    debug!("Processing complete")
}
