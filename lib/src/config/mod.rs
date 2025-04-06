//! Config: settings and options for a Whetstone project which may be serialized/deserialized.

use std::{fs, io};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "whetstone.yml";

/// A representation of a valid Whetstone project.
#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
}

impl Project {
    pub fn read(root_path: &PathBuf) -> io::Result<Self> {
        let whetstone_config = fs::read_dir(root_path)?.find(|item| {
            match item {
                Ok(entry) => entry.file_name() == CONFIG_FILE,
                Err(_) => false,
            }
        }).ok_or(io::Error::new(io::ErrorKind::NotFound, format!("{} not found in {}", CONFIG_FILE, root_path.display())))?;

        let project: Project = serde_yaml::from_str(fs::read_to_string(whetstone_config.unwrap().path())?.as_str()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Ill-formed project config file!")
        })?;

        Ok(project)
    }
}
