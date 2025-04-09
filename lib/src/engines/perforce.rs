use crate::{config, Engine, EngineWorker};

#[cfg(feature = "with-perforce")]
pub struct Perforce {
    config: config::perforce::StreamDefinition,
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
        todo!()
    }
}
