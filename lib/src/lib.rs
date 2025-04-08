use std::io;
use std::path::Path;
use crate::config::module::Module;
use crate::config::Project;

pub mod config;

struct Version(String);

pub fn open_project<P: AsRef<Path>>(root: P) -> io::Result<Project> {
    Project::read_from_config(root)
}

trait Engine {
    fn new(engine: &config::module::Engine) -> io::Result<Box<dyn Engine>>;
    fn sync(&self, version: Version, force: bool) -> io::Result<()>;
}



pub fn sync(project: &Project, version: Version, force: bool) -> io::Result<()> {
    for module in &project.modules {

    }
    Ok(())
}
