use std::io;
use std::io::ErrorKind::InvalidData;
use std::path::PathBuf;
use std::sync::Arc;
use crate::ModuleVersion;
use crate::config::rdedup;
use crate::config::rdedup::{CachingStrategy, Repository};
use crate::engines::{Engine, EngineWorker};

#[cfg(feature = "with-rdedup")]
pub struct Rdedup {
    config: rdedup::Config,
}

#[cfg(feature = "with-rdedup")]
pub struct RdedupWorker {
    config: rdedup::Config,
    rdedup_repo: rdedup_lib::Repo,
    decrypt_handle: rdedup_lib::DecryptHandle
}

impl Rdedup {
    pub fn new(config: rdedup::Config) -> Box<Self> {
        Box::new(Rdedup{
            config,
        })
    }
}

fn get_repo(config: Repository) -> io::Result<rdedup_lib::Repo> {
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

fn read_password_stub() -> io::Result<String> {
    Ok("".to_owned())
}

impl Engine for Rdedup {
    fn new_worker(&self) -> io::Result<Box<dyn EngineWorker>> {
        let repo = get_repo(self.config.repository.clone())?;
        // TODO actually support decryption
        let decrypt_handle = repo.unlock_decrypt(&read_password_stub)?;
        Ok(Box::new(RdedupWorker{
            config: self.config.clone(),
            rdedup_repo: repo,
            decrypt_handle,
        }))
    }
}

impl EngineWorker for RdedupWorker {
    fn sync(&self, root: &PathBuf, version: &ModuleVersion, _force: bool) -> io::Result<()> {
        let (read, mut write) = pipe::pipe();
        self.rdedup_repo.read(version.get_version(), &mut write, &self.decrypt_handle)?;
        tar::Archive::new(read).unpack(root.join(&self.config.path))
    }

    fn test_connection(&self) -> io::Result<()> {
        self.rdedup_repo.list_names().map(|_| ())
    }
}
