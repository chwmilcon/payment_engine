use env_logger::Builder;
#[allow(unused)] // TODO: remove after development
use log::{debug, error, info};
use clap::Parser;
mod transaction;
mod ledger;
mod account;
mod args;
use crate::args::Args;

fn main() {

   let args = Args::parse();

    // Set up the logger at the debug level if the debug flag is present
    if args.debug {
        Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    }


    debug!("processing");

    if let Err(e) = transaction::process_file(args.name.to_str().unwrap(), |_| Ok(()), args.stop_on_error) {
        error!("Error processing CSV: {}", e);
        std::process::exit(1);
    }
    debug!("Processing complete")
}
