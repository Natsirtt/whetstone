//! Config: settings and options for a Whetstone project which may be serialized/deserialized.

pub mod module;
#[cfg(feature = "with-rdedup")]
pub mod rdedup;
#[cfg(feature = "with-perforce")]
pub mod perforce;

use std::{fs, io};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::config::module::Module;

const CONFIG_FILE: &str = "whetstone.yml";

/// A representation of a valid Whetstone project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub(crate) name: String,
    pub(crate) root: PathBuf,
    // Root modules initiate sync chains.
    pub(crate) root_modules: Vec<String>,
    pub(crate) modules: Vec<Module>,
}

impl Project {
    pub fn new<P: AsRef<Path>>(name: String, root: P, root_modules: Vec<String>, modules: Vec<Module>) -> io::Result<Self> {
        Ok(Project {
            name,
            root: dunce::canonicalize(root)?,
            root_modules,
            modules,
        })
    }

    pub fn read_from_config<P: AsRef<Path>>(root_path: P) -> io::Result<Self> {
        let whetstone_config = fs::read_dir(&root_path)?.find(|item| {
            match item {
                Ok(entry) => entry.file_name() == CONFIG_FILE,
                Err(_) => false,
            }
        }).ok_or(io::Error::new(io::ErrorKind::NotFound, format!("{} not found in {}", CONFIG_FILE, root_path.as_ref().display())))?;

        let project: Project = serde_yaml::from_str(fs::read_to_string(whetstone_config.unwrap().path())?.as_str()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Ill-formed project config file: {}", e))
        })?;

        Ok(project)
    }

    pub fn write_to_config(&self) -> io::Result<()> {
        fs::write(self.root.join(CONFIG_FILE), serde_yaml::to_string(self).map_err(|e|{
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to write project {} to disk: {}", self.name, e))
        })?.as_bytes())
    }
}
