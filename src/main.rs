use std::{io, process};
use clap::{Args, Parser, Subcommand};
use url::Url;
use whetstone_lib as whetstone;
use whetstone::config::Engine;
use whetstone::config::Module;
use whetstone::config::rdedup::{CachingStrategy, Repository};
use whetstone::config::perforce::StreamDefinition;
use whetstone_lib::{config, Project};
use whetstone_lib::config::Dependency;

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

    let foo = whetstone::config::Project::new("Nush".to_string(), config::Infrastructure::Default, vec![
            "Content".into(),
            "Binaries".into(),
    ])?;
    foo.write_to_config(&args.directory)?;
    let content_module = Module {
        name: "Content".into(),
        dependencies: vec![Dependency { module: "Binaries".into() }],
        engine: Engine::Perforce(StreamDefinition {
            port: "ssl:vcs.knifeedgestudios.com".into(),
            stream: "//nush/unstable/dev".into(),
        })
    };
    let binaries_module: Module = Module {
        name: "Binaries".into(),
        dependencies: vec![],
        engine: Engine::Rdedup(config::rdedup::Config {
            path: ".".into(),
            repository: Repository::HttpServer {
                url: Url::parse("https://buildstore.knifeedgestudios.com/nush/").unwrap().into(),
                caching_strategy: CachingStrategy::Local {
                    path: ".rdedup".into(),
                    max_size: 10737418240, // 10 GB
                }
            },
        })
    };
    // TODO instead of naming the config file from their engine-type, like perforce.yml, we should
    // give a module context too and it should be [module_name].yml; so we could have several perforce
    // modules for instance.
    content_module.write_to_config(&args.directory)?;
    binaries_module.write_to_config(&args.directory)?;

    match args.command {
        Command::Sync(sub_args) => {
            let project = Project::open(&args.directory)?;
            println!("{}", project);
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
