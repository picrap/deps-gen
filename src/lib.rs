mod test;
mod generator;
mod data;
mod configuration;

#[allow(dead_code)]

pub mod deps {
    use std::io::Error;
    use crate::configuration::Configuration;

    pub fn gen() -> Result<(), Error> {
        gen_with_configuration(Configuration::default())
    }

    pub fn gen_with_configuration(configuration: Configuration) -> Result<(), Error> {
        crate::generator::Generator::gen_with_configuration(configuration)
    }
}
