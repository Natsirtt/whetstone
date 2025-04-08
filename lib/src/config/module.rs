use serde::{Deserialize, Serialize};
use crate::config::rdedup;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Engine {
    #[cfg(feature = "with-rdedup")]
    Rdedup(rdedup::Repository),
    #[cfg(feature = "with-perforce")]
    Perforce {
        port: String,
        stream: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    pub name: String,
    #[serde(flatten)]
    pub engine: Engine,
}
