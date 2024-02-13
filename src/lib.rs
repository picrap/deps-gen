mod test;

mod deps {
    use std::collections::BTreeMap;
    use std::env::current_dir;
    use std::fs;
    use std::io::Error;
    use std::path::PathBuf;
    use cargo_lock::Lockfile;
    use handlebars::Handlebars;

    pub struct Configuration {
        pub template_file_path: Option<PathBuf>,
        pub template: Option<String>,

        pub cargo_lock_path: PathBuf,

        pub target_path: Option<PathBuf>,
    }

    impl Configuration {
        pub fn new() -> Self {
            Self {
                template_file_path: Some("deps.template.rs".into()),
                template: None,
                cargo_lock_path: "Cargo.lock".into(),
                target_path: None,
            }
        }

        pub fn target_path(&self) -> PathBuf {
            if let Some(target_path) = &self.target_path {
                target_path.clone()
            } else {
                let template_file_path = self.template_file_path.as_ref().unwrap();
                let template_file_path = template_file_path.to_str().unwrap();
                if !template_file_path.contains(".template.") {
                    panic!("When output is not specified, the input file must contain the pattern “.template.”");
                }
                let target_path = template_file_path.replace(".template.", ".");
                target_path.into()
            }
        }

        pub fn template(&self) -> String {
            if let Some(template) = &self.template {
                template.clone()
            } else {
                let path = &self.template_file_path.as_ref().expect("Either a template or template file must be specified");
                let template = fs::read_to_string(path).expect("Problem reading template file");
                template
            }
        }
    }

    pub fn gen() -> Result<(), Error> {
        gen_with_configuration(Configuration::new())
    }

    pub fn gen_with_configuration(configuration: Configuration) -> Result<(), Error> {
        if !should_generate(&configuration) {
            return Ok(());
        }
        let output = generate_output(&configuration);
        Ok(())
    }

    #[cfg(test)]
    pub fn debug_generate_output(configuration: &Configuration) -> String {
        generate_output(configuration)
    }

    fn generate_output(configuration: &Configuration) -> String {
        let _cd = current_dir().unwrap();
        let lock_file = Lockfile::load(&configuration.cargo_lock_path).expect("Can’t load lock file");
        let packages = &lock_file.packages;
        let template = configuration.template();
        let handlebars = Handlebars::new();
        let mut data = BTreeMap::new();
        data.insert("dependencies", packages);
        let output = handlebars.render_template(template.as_str(), &data);
        let output = output.expect("Template error");
        output
    }

    fn should_generate(configuration: &Configuration) -> bool {
        if let Ok( target_time) = fs::metadata(&configuration.target_path()) {
            let source_time = fs::metadata(&configuration.cargo_lock_path).expect("Can’t open lock file");
            source_time.modified().unwrap() > target_time.modified().unwrap()
        } else {
            true
        }
    }
}
