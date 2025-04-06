use std::{io, process};
use clap::{Parser, Subcommand};

use whetstone_lib as whetstone;
use whetstone::config::module::Engine;
use whetstone::config::module::Module;

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

    let foo = whetstone::config::Project::new("foobar".to_string(), args.directory.clone(), vec![
        Module {
            name: "Binaries".into(),
            engine: Engine::Rdedup
        }
    ])?;
    foo.write()?;

    match args.command {
        Command::Sync => {
            let project = whetstone::open_project(args.directory)?;
            println!("{:?}", project);
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
