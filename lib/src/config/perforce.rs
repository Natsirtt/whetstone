use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "with-perforce")]
pub struct StreamDefinition {
    pub port: String,
    pub stream: String,
}