use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::path::{Path, PathBuf};
use crate::config::ModuleID;
use crate::engines::{Engine, EngineWorker};

pub mod config;
pub mod engines;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Version(String);

/// A Version to sync to that was adapted for a data engine. The Version is the user-facing main version they are synced against,
/// whereas the ModuleVersion is the Version adapted for a specific data engine instance that it can understand and sync against,
/// which can be different from the main one (both in format, if a data engine represents versions differently; and in actual version,
/// if a data engine's version can support several of the main Version's values. For instance, in Unreal Engine a version of editor binaries
/// will support all the main versions of assets/content built on top of it, until the next code update is pushed).
pub struct ModuleVersion {
    /// The ModuleID this ModuleVersion is for
    module_id: ModuleID,
    version: Version,
}

impl ModuleVersion {
    pub fn get_version(&self) -> &str {
        &self.version.0.as_str()
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Project {
    root: PathBuf,
    config: config::Project,
    module_to_engine: HashMap<ModuleID, Box<dyn Engine>>,
    // A map from a module ID to the list of modules that _directly_ depend on it
    reverse_dependencies: HashMap<ModuleID, Vec<ModuleID>>,
}

// A live project, mapped onto a project root.
impl Project {
    pub fn open<P: AsRef<Path>>(root: P) -> io::Result<Self> {
        let config = config::Project::read_from_config(&root)?;

        let mut module_to_engine = HashMap::new();
        let mut reverse_dependencies = HashMap::new();
        for module_id in &config.modules {
            let module_config = config::Module::read_from_config(root.as_ref(), module_id)?;
            module_to_engine.insert(module_id.clone(), engines::new_engine(module_config.engine));
            reverse_dependencies.
        }

        Ok(Project {
            config,
            root: root.as_ref().to_path_buf(),
            module_to_engine,
            reverse_dependencies,
        })
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Whetstone project {} @{} ({} module(s) loaded)", self.config.name, self.root.to_string_lossy(), self.module_to_engine.len())
    }
}
