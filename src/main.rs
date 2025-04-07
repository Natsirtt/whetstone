use std::{io, process};
use clap::{Parser, Subcommand};
use url::Url;
use whetstone_lib as whetstone;
use whetstone::config::module::Engine;
use whetstone::config::module::Module;
use whetstone_lib::config::rdedup::{CachingStrategy, Repository};

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

    let foo = whetstone::config::Project::new("foobar".to_string(), args.directory.clone(), "Content".into(), vec![
        Module {
            name: "Content".into(),
            engine: Engine::Perforce {
                port: "ssl:vcs.knifeedgestudios.com".into(),
                stream: "//nush/unstable/dev".into(),
            },
            dependencies: vec!["Binaries".into()],
            scopes: vec!["Content/".into()]
        },
        Module {
            name: "Binaries".into(),
            engine: Engine::Rdedup(Repository::HttpServer {
                url: Url::parse("https://knifeedgestudios.com/nush/").unwrap().into(),
                caching_strategy: CachingStrategy::Local {
                    path: ".rdedup".into(),
                    max_size: 10737418240, // 10 GB
                },
            }),
            dependencies: vec![],
            scopes: vec!["Binaries/".into()]
        },
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
