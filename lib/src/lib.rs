use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

pub mod config;
pub mod engines;

struct Version(String);

pub struct Project {
    name: String,
    root: PathBuf,
    engines: HashMap<String, Vec<Box<dyn Engine>>>,
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
            engines: Default::default(),
        })
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }

    pub fn write_config(&self) -> io::Result<()> {
        todo!()
    }

    pub fn add_module(module: config::Module) -> io::Result<()> {
        todo!("Add a module but also create an engine for it, if that cannot happen the module is ill-configured and will not be added")
    }
}

pub trait Engine: Send + Sync {
    fn new_worker(&self) -> io::Result<Box<dyn EngineWorker>>;
}

/// A Version to sync to that was adapted for a data engine. The Version is the user-facing main version they are synced against,
/// whereas the ModuleVersion is the Version adapted for a specific data engine instance that it can understand and sync against,
/// which can be different from the main one (both in format, if a data engine represents versions differently; and in actual version,
/// if a data engine's version can support several of the main Version's values. For instance, in Unreal Engine a version of editor binaries
/// will support all the main versions of assets/content built on top of it, until the next code update is pushed).
struct ModuleVersion(String);

pub trait EngineWorker: Send {
    fn sync(&self, root: &PathBuf, version: ModuleVersion, force: bool) -> io::Result<()>;

    // TODO start writing up a Whetstone error domain instead of using io everywhere?
    fn test_connection(&self) -> io::Result<()>;
}
