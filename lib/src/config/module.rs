use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Engine {
    #[cfg(feature = "with-rdedup")]
    Rdedup,
    #[cfg(feature = "with-perforce")]
    Perforce,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub engine: Engine,
}
