use std::io;
use std::path::PathBuf;
use crate::config::Project;

mod config;

pub fn open_project(root: &PathBuf) -> io::Result<Project> {
    Project::read(root)
}
