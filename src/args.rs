#[allow(unused)] // TODO: remove after development
use log::{debug, error, info};
use clap::Parser;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "File to process")]    
    pub name: PathBuf,
    #[arg(help = "Turn on debug logging")]
    #[clap(long, short = 'd')]
    pub debug: bool,
    #[arg(help = "Should we stop everything when there's a processing error? (Default:false)")]
    #[arg(default_value_t = false)]
    #[clap(long)]
    pub stop_on_error: bool,
    #[arg(help = "file to write log messages into.")]
    #[clap(long)]
    pub logfile: Option<String>,
    #[arg(help = "File to dump the internal ledger, all data")]
    #[clap(long)]
    pub statelog: Option<String>,
}
