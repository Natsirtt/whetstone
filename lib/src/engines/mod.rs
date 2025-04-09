#[cfg(feature = "with-rdedup")]
mod rdedup;
#[cfg(feature = "with-perforce")]
mod perforce;

use crate::config;
use crate::engines::perforce::Perforce;
use crate::engines::rdedup::Rdedup;

pub fn new_engine(engine: config::Engine) -> Box<dyn crate::Engine> {
    match engine {
        config::Engine::Rdedup(config) => {
            Rdedup::new(config)
        }
        config::Engine::Perforce(config) => {
            Perforce::new(config)
        }
    }
}