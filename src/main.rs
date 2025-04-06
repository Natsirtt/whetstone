use std::{io, process};
use std::path::PathBuf;
use clap::{Parser, Subcommand};

use whetstone_lib as whetstone;

#[derive(Debug, Parser)]
#[clap(about = "Heterogeneous project dependencies manager. Keep your projects sharply up-to-date!")]
struct Args {
    #[clap(name = "dir", short, long, default_value = ".")]
    directory: std::ffi::OsString,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
//#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
enum Command {
    Sync,
}

fn run() -> io::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Sync => {
            let project = whetstone::open_project(&PathBuf::from(args.directory))?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
