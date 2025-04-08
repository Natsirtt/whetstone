use std::{fs, io};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;
use crate::config::{rdedup, Project};

#[derive(Debug, Serialize, Deserialize, Clone, IntoStaticStr)]
pub enum Engine {
    #[cfg(feature = "with-rdedup")]
    Rdedup(rdedup::Repository),
    #[cfg(feature = "with-perforce")]
    Perforce {
        port: String,
        stream: String,
    },
}

impl Engine {
    fn get_config_file(&self, project_context: &Project) -> PathBuf {
        let as_string: &'static str = self.into();
        project_context.root.join(format!("{}.yml", as_string))
    }
    pub fn read_from_config(&self, project_context: &Project) -> io::Result<Self> {
        let file = self.get_config_file(project_context);
        if !Path::is_file(&file) {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not find file {}", file.to_string_lossy())));
        }
        let engine: Engine = serde_yaml::from_str(fs::read_to_string(file)?.as_str()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Ill-formed module engine config file: {}", e))
        })?;

        Ok(engine)
    }

    pub fn write_to_config(&self, project_context: &Project) -> io::Result<()> {
        let file = self.get_config_file(project_context);
        fs::write(file, serde_yaml::to_string(self).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to write module engine config to disk: {}", e))
        })?.as_bytes())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    pub name: String,
}
