use std::{io, process};
use clap::{Args, Parser, Subcommand};
use url::Url;
use whetstone_lib as whetstone;
use whetstone::config::module::Engine;
use whetstone::config::module::Module;
use whetstone_lib::config::rdedup::{CachingStrategy, Repository};

#[derive(Debug, Parser)]
#[clap(about = "Heterogeneous project dependencies manager. Keep your projects sharply up-to-date!")]
struct CliArgs {
    #[clap(name = "dir", short, long, default_value = ".")]
    directory: std::ffi::OsString,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
struct SyncArgs {
    #[clap(short, help = "Forces all components to forcibly sync to this version even if they already think they are up-to-date")]
    force: bool,
}

#[derive(Debug, Subcommand)]
//#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
enum Command {
    Sync(SyncArgs),
}

fn run() -> io::Result<()> {
    let args = CliArgs::parse();

    let foo = whetstone::config::Project::new("foobar".to_string(), args.directory.clone(), vec!["Content".into()], vec![
            "Content".into(),
            "Binaries".into(),
    ])?;
    foo.write_to_config()?;
    let content_module = Module {
        name: "Content".into(),
        engine: Engine::Perforce {
            port: "ssl:vcs.knifeedgestudios.com".into(),
            stream: "//nush/unstable/dev".into(),
        }
    };
    let binaries_engine = Engine::Rdedup(Repository::HttpServer {
        url: Url::parse("https://knifeedgestudios.com/nush/").unwrap().into(),
        caching_strategy: CachingStrategy::Local {
            path: ".rdedup".into(),
            max_size: 10737418240, // 10 GB
        },
    });
    // TODO instead of naming the config file from their engine-type, like perforce.yml, we should
    // give a module context too and it should be [module_name].yml; so we could have several perforce
    // modules for instance.
    content_engine.write_to_config(&foo)?;
    binaries_engine.write_to_config(&foo)?;

    match args.command {
        Command::Sync(sub_args) => {
            let project = whetstone::open_project(args.directory)?;
            println!("{:?}", project);
            println!("Sync. Force? {}", sub_args.force);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Whetstone exited with error: {}", e);
        process::exit(1);
    }
}
