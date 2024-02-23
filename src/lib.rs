#![allow(non_ascii_idents)]

mod test;
mod generator;
mod data;
pub mod configuration;

#[allow(dead_code)]

pub mod deps {
    use std::fs;
    use std::io::Error;
    use crate::configuration::{Configuration, TemplateSource};

    pub fn gen() -> Result<(), Error> {
        gen_with_configuration(Configuration::default())
    }

    pub fn gen_with_configuration(configuration: Configuration) -> Result<(), Error> {
        if let TemplateSource::File(source_path) = &configuration.template {
            println!("cargo:rerun-if-changed={}", fs::canonicalize(source_path).unwrap().to_str().unwrap());
        }
        crate::generator::Generator::gen_with_configuration(configuration)
    }
}
