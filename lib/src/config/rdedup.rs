use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CachingStrategy {
    None,
    Local {
        path: PathBuf,
        max_size: u64,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Repository {
    Directory {
        path: PathBuf,
    },
    HttpServer {
        url: String,
        caching_strategy: CachingStrategy,
    },
}