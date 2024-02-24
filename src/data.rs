use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::RwLock;
use cargo_metadata::{MetadataCommand, Package};
use serde::Serialize;
use crate::deps::Configuration;

#[derive(Serialize)]
pub struct Data {
    dependencies: Vec<Package>,
}

struct PackageNode {
    pub package: Rc<RwLock<Package>>,
    pub dependencies: RwLock<Vec<Rc<RwLock<PackageNode>>>>,
    referenced: usize,
}

impl Data {
    pub fn load(configuration: &Configuration) -> Self {
        let metadata = MetadataCommand::new().exec().expect("Dood?");
        let tree = Self::create_tree(metadata.packages.iter().collect());
        let packages = Self::flatten(tree, configuration);
        let packages = packages.iter().map(|p| p.read().unwrap().package.read().unwrap().clone()).collect();
        Self {
            dependencies: packages,
        }
    }

    fn create_tree(packages: Vec<&Package>) -> Rc<RwLock<PackageNode>> {
        let mut tree = BTreeMap::<&str, Rc<RwLock<PackageNode>>>::new();
        for package in packages {
            tree.insert(package.name.as_str(), Rc::new(RwLock::new(PackageNode {
                package: Rc::new(RwLock::new(package.clone())),
                dependencies: RwLock::new(vec![]),
                referenced: 0,
            })));
        }
        for value in tree.values() {
            let mut dependencies: Vec<Rc<RwLock<PackageNode>>> = value.read().unwrap().package.read().unwrap().dependencies.iter()
                .filter_map(|d| tree.get(d.name.as_str())).cloned().collect();
            for dependency in dependencies.iter() {
                dependency.write().unwrap().referenced += 1;
            }
            value.read().unwrap().dependencies.write().unwrap().append(&mut dependencies);
        }
        let root = tree.values().filter(|k| k.read().unwrap().referenced == 0).last().unwrap();
        root.clone()
    }

    fn flatten(tree: Rc<RwLock<PackageNode>>, configuration: &Configuration) -> Vec<Rc<RwLock<PackageNode>>> {
        let mut packages: BTreeMap<String, Rc<RwLock<PackageNode>>> = BTreeMap::new();
        Self::flatten_to_vec(tree, configuration, 0, &mut packages);
        packages.values().cloned().collect()
    }

    fn flatten_to_vec(tree: Rc<RwLock<PackageNode>>, configuration: &Configuration, current_depth: usize, packages: &mut BTreeMap<String, Rc<RwLock<PackageNode>>>) {
        if let Some(maximum_depth) = configuration.maximum_depth {
            if current_depth >= maximum_depth {
                return;
            }
        }
        let package_name = tree.read().unwrap().package.read().unwrap().name.clone();
        if packages.contains_key(&package_name) {
            return;
        }
        if current_depth > 0 || configuration.include_root {
            packages.insert(package_name, tree.clone());
        }

        for dependency in tree.read().unwrap().dependencies.read().unwrap().iter() {
            Self::flatten_to_vec(dependency.clone(), configuration, current_depth + 1, packages);
        }
    }
}
