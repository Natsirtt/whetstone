use std::path::PathBuf;
use p4_cmd::P4;
use crate::{config, Engine, EngineWorker, ModuleVersion};

// TODO get client from a user-config; but not from whetstone shared config! And possibly other perforce settings why not.
// TODO ditch p4_cmd lib which is weirdly designed and seems unmaintained; or bring it up to shape in fork

#[cfg(feature = "with-perforce")]
pub struct Perforce {
    config: config::perforce::StreamDefinition,
}

#[cfg(feature = "with-perforce")]
pub struct PerforceWorker {
    p4: P4,
}

impl Perforce {
    pub fn new(config: config::perforce::StreamDefinition) -> Box<Self> {
        Box::new(Perforce{
            config
        })
    }
}

impl Engine for Perforce {
    fn new_worker(&self) -> std::io::Result<Box<dyn EngineWorker>> {
        // TODO test perforce connection, maybe with `p4 info`? Check that the workspace root is within this whetstone project
        Ok(Box::new(PerforceWorker {
            p4: P4::new().set_port(Some(self.config.port.clone()))
        }))
    }
}

impl EngineWorker for PerforceWorker {
    fn sync(&self, root: &PathBuf, version: &ModuleVersion, force: bool) -> std::io::Result<()> {
        // TODO whetstone error domain
        self.p4.sync(root.join(format!("...@{}", version.get_version())).to_str().unwrap()).force(force).run().map(|_| ()).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn test_connection(&self) -> std::io::Result<()> {
        // TODO p4 info
        Ok(())
    }
}
