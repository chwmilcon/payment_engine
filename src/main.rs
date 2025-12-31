use env_logger::Builder;
use log::debug;


use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    name: PathBuf,
    #[clap(long, short = 'd')]
    debug: bool,
}
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
 
    debug!("Hello, world!");
 
    println!("Hello, world!");

    println!("Filename: {}", args.name.display());
}
