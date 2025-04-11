#[cfg(feature = "with-rdedup")]
mod rdedup;
#[cfg(feature = "with-perforce")]
mod perforce;

use std::io;
use std::path::PathBuf;
use crate::{config, ModuleVersion};
use crate::engines::perforce::Perforce;
use crate::engines::rdedup::Rdedup;

pub trait Engine: Send + Sync {
    fn new_worker(&self) -> io::Result<Box<dyn EngineWorker>>;
}

pub trait EngineWorker: Send {
    fn sync(&self, root: &PathBuf, version: &ModuleVersion, force: bool) -> io::Result<()>;

    // TODO start writing up a Whetstone error domain instead of using io everywhere?
    fn test_connection(&self) -> io::Result<()>;
}

pub fn new_engine(engine: config::Engine) -> Box<dyn Engine> {
    match engine {
        config::Engine::Rdedup(config) => {
            Rdedup::new(config)
        }
        config::Engine::Perforce(config) => {
            Perforce::new(config)
        }
    }
}