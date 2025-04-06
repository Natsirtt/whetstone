use std::io;
use std::path::Path;
use crate::config::Project;

pub mod config;

pub fn open_project<P: AsRef<Path>>(root: P) -> io::Result<Project> {
    Project::read(root)
}
