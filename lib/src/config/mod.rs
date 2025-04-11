//! Config: settings and options for a Whetstone project which may be serialized/deserialized.

#[cfg(feature = "with-rdedup")]
pub mod rdedup;
#[cfg(feature = "with-perforce")]
pub mod perforce;

use std::{fs, io};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use serde::{Deserialize, Serialize};

const DIRECTORY: &str = ".whetstone";
const TMP_DIRECTORY: &str = "tmp";
const CONFIG_FILE: &str = "Whetstone.yml";

pub fn get_directory<P: AsRef<Path>>(root: P) -> PathBuf {
    root.as_ref().join(DIRECTORY)
}

fn get_main_file<P: AsRef<Path>>(root: P) -> PathBuf {
    get_directory(root).join(CONFIG_FILE)
}

fn get_temp_directory<P: AsRef<Path>>(root: P) -> PathBuf {
    get_directory(root).join(DIRECTORY).join(TMP_DIRECTORY)
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ModuleID(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfrastructureID(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Infrastructure {
    Default,
    Custom(InfrastructureID),
}

/// A representation of a valid Whetstone project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub(crate) name: String,
    pub(crate) infrastructure: Infrastructure,
    pub(crate) main_module: ModuleID,
    pub(crate) modules: Vec<ModuleID>,
}

impl Project {
    pub fn new(name: String, infrastructure: Infrastructure, main_module: ModuleID, modules: Vec<ModuleID>) -> io::Result<Self> {
        Ok(Project {
            name,
            infrastructure,
            main_module,
            modules,
        })
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

    pub fn write_to_config<P: AsRef<Path>>(&self, root_path: P) -> io::Result<()> {
        let binding = get_directory(&root_path);
        let directory = binding.as_path();
        if !Path::is_dir(directory) {
            if Path::is_file(directory) {
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Cannot create {} directory over existing file", directory.to_string_lossy())));
            }
            fs::create_dir_all(directory)?;
        }
        assert!(Path::is_dir(directory));
        fs::write(get_main_file(&root_path), serde_yaml::to_string(self).map_err(|e|{
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to write project {} to disk: {}", self.name, e))
        })?.as_bytes())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Engine {
    #[cfg(feature = "with-rdedup")]
    Rdedup(rdedup::Config),
    #[cfg(feature = "with-perforce")]
    Perforce(perforce::StreamDefinition),
}

impl Display for ModuleID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ModuleID {
    fn from(value: &str) -> Self {
        ModuleID(value.to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    pub name: ModuleID,
    pub engine: Engine,
}

impl Module {
    fn get_config_file<P: AsRef<Path>>(root_path: P, id: &ModuleID) -> PathBuf {
        get_directory(root_path).join(format!("{}.yml", id.0.as_str()))
    }
    pub fn read_from_config<P: AsRef<Path>>(root_path: P, id: &ModuleID) -> io::Result<Self> {
        let file = Module::get_config_file(root_path, id);
        if !Path::is_file(&file) {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not find file {}", file.to_string_lossy())));
        }
        let module: Module = serde_yaml::from_str(fs::read_to_string(file)?.as_str()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Ill-formed module config file: {}", e))
        })?;

        Ok(module)
    }

    pub fn write_to_config<P: AsRef<Path>>(&self, root_path: P) -> io::Result<()> {
        let file = Module::get_config_file(root_path, &self.name);
        fs::write(file, serde_yaml::to_string(self).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to write module config to disk: {}", e))
        })?.as_bytes())
    }
}
