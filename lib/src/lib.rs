use std::io;
use std::path::Path;
use crate::config::Project;

pub mod config;
pub mod engines;

struct Version(String);

pub fn open_project<P: AsRef<Path>>(root: P) -> io::Result<Project> {
    Project::read_from_config(root)
}

pub trait Engine: Send + Sync {
    fn new_worker(&self) -> io::Result<Box<dyn EngineWorker>>;
}

pub trait EngineWorker: Send {
    fn sync(&self, version: Version, force: bool) -> io::Result<()>;
}
