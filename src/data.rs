use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::RwLock;
use cargo_lock::{Lockfile, Package};
use crates_io_api::{CrateResponse, SyncClient};
use serde::Serialize;
use crate::configuration::Configuration;

#[derive(Serialize)]
pub struct Data {
    dependencies: Vec<Package>,
}

struct PackageNode<'α> {
    pub package: &'α Package,
    pub dependencies: RwLock<Vec<Rc<RwLock<PackageNode<'α>>>>>,
    referenced: usize,
}

impl Data {
    pub fn load(configuration: &Configuration) -> Self {
        let lock_file = Lockfile::load(&configuration.cargo_lock_path).expect("Can’t load lock file");
        let names: Vec<&str> = lock_file.packages.iter().map(|p| p.name.as_str()).collect();
        let tree = Self::create_tree(lock_file.packages.iter().collect());
        let crates = Self::load_crates(names);
        let packages = lock_file.packages;
        Self {
            dependencies: packages,
        }
    }

    fn create_tree(packages: Vec<&Package>) -> Rc<RwLock<PackageNode>> {
        let mut tree = BTreeMap::<&str, Rc<RwLock<PackageNode>>>::new();
        for package in &packages {
            tree.insert(package.name.as_str(), Rc::new(RwLock::new(PackageNode {
                package: package,
                dependencies: RwLock::new(vec![]),
                referenced: 0,
            })));
        }
        for (_, value) in tree.iter() {
            let mut dependencies: Vec<Rc<RwLock<PackageNode>>> = value.read().unwrap().package.dependencies.iter().map(|d| tree.get(d.name.as_str()).unwrap().clone()).collect();
            for dependency in dependencies.iter() {
                dependency.write().unwrap().referenced += 1;
            }
            value.read().unwrap().dependencies.write().unwrap().append(&mut dependencies);
        }
        let root = tree.values().filter(|k| k.read().unwrap().referenced == 0).last().unwrap();
        root.clone()
    }

    fn load_crates(names: Vec<&str>) -> Vec<CrateResponse> {
        let client = SyncClient::new("deps-gen", std::time::Duration::from_millis(1000), ).unwrap();
        names.iter().map(|n| client.get_crate(n).expect("Can’t load crate info")).collect()
    }
}
