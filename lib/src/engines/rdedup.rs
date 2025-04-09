use std::io;
use std::io::ErrorKind::InvalidData;
use std::sync::Arc;
use crate::{Engine, EngineWorker, Version};
use crate::config::rdedup::{CachingStrategy, Repository};

#[cfg(feature = "with-rdedup")]
pub struct Rdedup {
    config: Repository,
}

#[cfg(feature = "with-rdedup")]
pub struct RdedupWorker {
    config: Repository,
    rdedup_repo: rdedup_lib::Repo,
}

impl Rdedup {
    pub fn new(config: Repository) -> Box<Self> {
        Box::new(Rdedup{
            config,
        })
    }
}

fn get_repo(config: Repository) -> std::io::Result<rdedup_lib::Repo> {
    rdedup_lib::Repo::open(Arc::new(move || {
        match &config {
            Repository::Directory { path } => {
                Ok(Box::new(rdedup_lib::backends::local::Local::new(path.clone())))
            }
            Repository::HttpServer { url, caching_strategy } => {
                let http_backend = Box::new(rdedup_lib::backends::http::HttpReadOnly::new(url.parse().map_err(|e| {
                    io::Error::new(InvalidData, e)
                })?));
                match caching_strategy {
                    CachingStrategy::None => Ok(http_backend),
                    CachingStrategy::Local { path, max_size: _max_size } => {
                        Ok(Box::new(rdedup_lib::backends::local_cache::LocalCache::new(path.into(), http_backend)))
                    }
                }
            }
        }
    }), None) // TODO add logging throughout whetstone; and forward it here :)
}

impl Engine for Rdedup {
    fn new_worker(&self) -> io::Result<Box<dyn EngineWorker>> {
        Ok(Box::new(RdedupWorker{
            config: self.config.clone(),
            rdedup_repo: get_repo(self.config.clone())?
        }))
    }
}

impl EngineWorker for RdedupWorker {
    fn sync(&self, version: Version, force: bool) -> std::io::Result<()> {
        todo!()
    }
}
