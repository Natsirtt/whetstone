use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg(feature = "with-rdedup")]
pub enum CachingStrategy {
    None,
    Local {
        path: PathBuf,
        max_size: u64,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg(feature = "with-rdedup")]
pub enum Repository {
    Directory {
        path: PathBuf,
    },
    HttpServer {
        url: String,
        caching_strategy: CachingStrategy,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg(feature = "with-rdedup")]
pub struct Config {
    /// The root directory where the reconstituted tar file will be unarchived at, relative to the root of the project.
    /// So to untar at the root, use "."
    pub root: PathBuf,
    /// The rdedup repository to get data from
    pub repository: Repository,
}
