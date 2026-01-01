#[allow(unused)] // TODO: remove after development
use log::{debug, error, info};
use clap::Parser;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub name: PathBuf,
    #[clap(long, short = 'd')]
    pub debug: bool,
    #[arg(default_value_t = false)]
    #[clap(long)]
    pub stop_on_error: bool,
    #[clap(long)]
    pub logfile: Option<String>,
}
