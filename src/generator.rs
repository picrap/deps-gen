use std::env::current_dir;
use std::fs;
use std::io::Error;
use handlebars::Handlebars;
use crate::configuration::Configuration;
use crate::data::Data;

pub(crate) struct Generator;

impl Generator {
    pub fn gen_with_configuration(configuration: Configuration) -> Result<(), Error> {
        if !Self::should_generate(&configuration) {
            return Ok(());
        }
        let output = Self::generate_output(&configuration);
        fs::write(configuration.target_path(), output)?;
        Ok(())
    }

   pub(crate) fn generate_output(configuration: &Configuration) -> String {
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
