//! Config: settings and options for a Whetstone project which may be serialized/deserialized.

#[cfg(feature = "with-rdedup")]
pub mod rdedup;
#[cfg(feature = "with-perforce")]
pub mod perforce;

use std::{fs, io};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

const DIRECTORY: &str = ".whetstone";
const CONFIG_FILE: &str = "Whetstone.yml";

/// A representation of a valid Whetstone project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub(crate) name: String,
    pub(crate) root: PathBuf,
    // Root modules initiate sync chains.
    pub(crate) root_modules: Vec<String>,
    pub(crate) modules: Vec<String>,
}

impl Project {
    pub fn new<P: AsRef<Path>>(name: String, root: P, root_modules: Vec<String>, modules: Vec<String>) -> io::Result<Self> {
        Ok(Project {
            name,
            root: dunce::canonicalize(root)?,
            root_modules,
            modules,
        })
    }

    pub fn get_config_directory(&self) -> PathBuf {
        self.root.join(DIRECTORY)
    }

    fn get_config_file(&self) -> PathBuf {
        self.get_config_directory().join(CONFIG_FILE)
    }

    pub fn read_from_config<P: AsRef<Path>>(root_path: P) -> io::Result<Self> {
        let config_file = PathBuf::from(root_path.as_ref()).join(DIRECTORY).join(CONFIG_FILE);
        if !Path::is_file(&config_file) {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not find file {}", config_file.to_string_lossy())));
        }

        let project: Project = serde_yaml::from_str(fs::read_to_string(config_file)?.as_str()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Ill-formed project config file: {}", e))
        })?;

        Ok(project)
    }

    pub fn write_to_config(&self) -> io::Result<()> {
        let binding = self.get_config_directory();
        let directory = binding.as_path();
        if !Path::is_dir(directory) {
            if Path::is_file(directory) {
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Cannot create {} directory over existing file", directory.to_string_lossy())));
            }
            fs::create_dir_all(directory)?;
        }
        assert!(Path::is_dir(directory));
        fs::write(self.get_config_file(), serde_yaml::to_string(self).map_err(|e|{
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to write project {} to disk: {}", self.name, e))
        })?.as_bytes())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Engine {
    #[cfg(feature = "with-rdedup")]
    Rdedup(rdedup::Repository),
    #[cfg(feature = "with-perforce")]
    Perforce(perforce::StreamDefinition),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    pub name: String,
    pub engine: Engine,
}

impl Module {
    fn get_config_file(&self, project_context: &Project) -> PathBuf {
        project_context.get_config_directory().join(format!("{}.yml", self.name))
    }
    pub fn read_from_config(&self, project_context: &Project) -> io::Result<Self> {
        let file = self.get_config_file(project_context);
        if !Path::is_file(&file) {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not find file {}", file.to_string_lossy())));
        }
        let module: Module = serde_yaml::from_str(fs::read_to_string(file)?.as_str()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Ill-formed module config file: {}", e))
        })?;

        Ok(module)
    }

    pub fn write_to_config(&self, project_context: &Project) -> io::Result<()> {
        let file = self.get_config_file(project_context);
        fs::write(file, serde_yaml::to_string(self).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to write module config to disk: {}", e))
        })?.as_bytes())
    }
}
