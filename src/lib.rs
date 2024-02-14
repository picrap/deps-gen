mod test;

#[allow(dead_code)]

mod deps {
    use std::env::current_dir;
    use std::fs;
    use std::io::Error;
    use std::path::PathBuf;
    use cargo_lock::{Lockfile, Package};
    use handlebars::Handlebars;
    use serde::Serialize;

    pub enum TemplateSource {
        Text(String),
        File(PathBuf)
    }

    pub struct Configuration {
        pub template: TemplateSource,
        pub cargo_lock_path: PathBuf,
        pub target_path: Option<PathBuf>,
        pub post_template_search: Option<String>,
        pub post_template_replace: String,
        pub include_root: bool,
        pub maximum_depth: Option<usize>,
    }

    impl Configuration {
        pub fn new() -> Self {
            Self {
                template: TemplateSource::File("deps.template.rs".into()),
                cargo_lock_path: "Cargo.lock".into(),
                target_path: None,
                post_template_search: Some("//{}".into()),
                post_template_replace: "".into(),
                include_root: false,
                maximum_depth: None,
            }
        }

        pub fn target_path(&self) -> PathBuf {
            if let Some(target_path) = &self.target_path {
                target_path.clone()
            } else if let TemplateSource::File(template_file) = &self.template {
                let template_file_path = template_file.to_str().unwrap();
                if !template_file_path.contains(".template.") {
                    panic!("When output is not specified, the input file must contain the pattern “.template.”");
                }
                let target_path = template_file_path.replace(".template.", ".");
                target_path.into()
            } else {
                panic!("Can’t guess output file!")
            }
        }

        pub fn template_text(&self) -> String {
            match &self.template {
                TemplateSource::Text(text) => text.clone(),
                TemplateSource::File(template_file) => {
                    fs::read_to_string(template_file).expect("Problem reading template file")
                },
            }
        }
    }

    #[derive(Serialize)]
    struct Data {
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

    pub fn gen() -> Result<(), Error> {
        gen_with_configuration(Configuration::new())
    }

    pub fn gen_with_configuration(configuration: Configuration) -> Result<(), Error> {
        if !should_generate(&configuration) {
            return Ok(());
        }
        let output = generate_output(&configuration);
        fs::write(configuration.target_path(), output)?;
        Ok(())
    }

    #[cfg(test)]
    pub fn debug_generate_output(configuration: &Configuration) -> String {
        generate_output(configuration)
    }

    fn generate_output(configuration: &Configuration) -> String {
        let _cd = current_dir().unwrap();
        let template = configuration.template_text();
        let handlebars = Handlebars::new();
        let data = Data::load(configuration);
        let output = handlebars.render_template(template.as_str(), &data);
        let mut output = output.expect("Template error");
        if let Some(post_template_search) = &configuration.post_template_search {
            output = output.replace(post_template_search, &configuration.post_template_replace)
        }
        output
    }

    fn should_generate(configuration: &Configuration) -> bool {
        if let Ok(target_time) = fs::metadata(configuration.target_path()) {
            let source_time = fs::metadata(&configuration.cargo_lock_path).expect("Can’t open lock file");
            source_time.modified().unwrap() > target_time.modified().unwrap()
        } else {
            true
        }
    }
}
