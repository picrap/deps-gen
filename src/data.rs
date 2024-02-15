
use cargo_lock::{Lockfile, Package};
use serde::Serialize;
use crate::configuration::Configuration;

#[derive(Serialize)]
pub struct Data {
    dependencies: Vec<Package>,
}

impl Data {
    pub fn load(configuration: &Configuration) -> Self {
        let lock_file = Lockfile::load(&configuration.cargo_lock_path).expect("Can’t load lock file");
        let packages = lock_file.packages;
        Self {
            dependencies: packages,
        }
    }
}

