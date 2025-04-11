use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};
use crate::engines::{Engine, EngineWorker};

pub mod config;
pub mod engines;
pub mod infrastructure;
use infrastructure::ModuleVersion;

struct Version(String);

pub struct Project {
    name: String,
    root: PathBuf,
    module_to_engine: HashMap<String, Box<dyn Engine>>,
}

impl Project {
    pub fn new<P: AsRef<Path>>(root: P) -> io::Result<Project> {
        todo!("Implement a project-seeding so people can start configuring it")
    }

    pub fn open<P: AsRef<Path>>(root: P) -> io::Result<Project> {
        let config = config::Project::read_from_config(&root)?;
        Ok(Project {
            name: config.name,
            root: root.as_ref().to_path_buf(),
            module_to_engine: Default::default(),
        })
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }

    pub fn write_config(&self) -> io::Result<()> {
        todo!()
    }
}
