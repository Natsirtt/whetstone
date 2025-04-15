use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::path::{Path, PathBuf};
use crate::engines::{Engine, EngineWorker};

pub mod config;
pub mod engines;
pub mod infrastructure;
use infrastructure::ModuleVersion;
use crate::infrastructure::Infrastructure;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Version(String);

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Project {
    root: PathBuf,
    config: config::Project,
    infrastructure: Box<dyn Infrastructure>,
    module_to_engine: HashMap<String, Box<dyn Engine>>,
}

// A live project, mapped onto a project root.
impl Project {
    pub fn open<P: AsRef<Path>>(root: P) -> io::Result<Self> {
        let config = config::Project::read_from_config(&root)?;

        let mut module_to_engine = HashMap::new();
        for module_id in &config.modules {
            let module_config = config::Module::read_from_config(root.as_ref(), module_id)?;
            module_to_engine.insert(module_id.to_string(), engines::new_engine(module_config.engine));
        }

        let infrastructure = infrastructure::new_infrastructure(&config.infrastructure)?;

        Ok(Project {
            config,
            root: root.as_ref().to_path_buf(),
            infrastructure,
            module_to_engine,
        })
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Whetstone project {} @{} ({} module(s) loaded on infrastructure {})", self.config.name, self.root.to_string_lossy(), self.module_to_engine.len(), self.infrastructure.get_name())
    }
}
