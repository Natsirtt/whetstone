use std::fmt::Display;
use std::io;
use crate::config::ModuleID;
use crate::{config, Version};

/// A Version to sync to that was adapted for a data engine. The Version is the user-facing main version they are synced against,
/// whereas the ModuleVersion is the Version adapted for a specific data engine instance that it can understand and sync against,
/// which can be different from the main one (both in format, if a data engine represents versions differently; and in actual version,
/// if a data engine's version can support several of the main Version's values. For instance, in Unreal Engine a version of editor binaries
/// will support all the main versions of assets/content built on top of it, until the next code update is pushed).
pub trait ModuleVersion: Display {
    /// The ModuleID this ModuleVersion is for
    fn get_module_id(&self) -> ModuleID;
}

pub trait Infrastructure {
    // TODO Whetstone error domain instead of generic io errors
    fn get_latest_version() -> io::Result<Version>;
    fn get_version_for_module(module: &ModuleID, version: &Version) -> Box<impl ModuleVersion>;
}