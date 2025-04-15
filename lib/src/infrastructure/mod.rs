use std::io;
use std::io::Error;
use crate::config::ModuleID;
use crate::{config, Version};

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

pub trait Infrastructure {
    // TODO Whetstone error domain instead of generic io errors
    fn get_name(&self) -> &'static str;
    fn get_version_for_module(&self, module: &ModuleID, version: &Version) -> io::Result<ModuleVersion>;
}

#[cfg(feature = "with-default-infrastructure")]
struct DatabaseInfrastructure {
}

const DEFAULT_INFRASTRUCTURE_NAME: &'static str = "Default-database";

impl Infrastructure for DatabaseInfrastructure {
    fn get_name(&self) -> &'static str {
        DEFAULT_INFRASTRUCTURE_NAME
    }



    fn get_version_for_module(&self, module: &ModuleID, version: &Version) -> io::Result<ModuleVersion> {
        Ok(ModuleVersion {
            module_id: module.clone(),
            version: version.clone(),
        })
    }
}

pub fn new_infrastructure(infrastructure: &config::Infrastructure) -> io::Result<Box<dyn Infrastructure>> {
    match infrastructure {
        config::Infrastructure::Default => {
            if cfg!(feature = "with-default-infrastructure") {
                Ok(Box::new(DatabaseInfrastructure {}))
            } else {
                Err(Error::new(io::ErrorKind::Unsupported, "Default infrastructure is not compiled in"))
            }
        }
        config::Infrastructure::Custom(_) => {
            Err(Error::new(io::ErrorKind::Unsupported, "Custom infrastructures are not supported yet"))
        }
    }
}
