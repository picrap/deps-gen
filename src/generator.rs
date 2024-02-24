use std::env::current_dir;
use std::fs;
use std::io::Error;
use std::time::SystemTime;
use handlebars::Handlebars;
use crate::data::Data;
use crate::{Configuration, TemplateSource};

pub(crate) struct Generator;

impl Generator {
    pub fn gen_with_configuration(configuration: Configuration) -> Result<(), Error> {
        if !Self::should_generate(&configuration) {
            println!("deps-gen: no need to generate dependencies");
            return Ok(());
        }
        println!("deps-gen: generating dependencies");
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
            let lock_file_time = fs::metadata(&configuration.cargo_lock_path).expect("Can’t open lock file");
            if lock_file_time.modified().unwrap() > target_time.modified().unwrap() {
                println!("deps-gen: lock is more recent");
                true
            } else if let TemplateSource::File(source_file) = &configuration.template {
                if let Ok(source_time) = fs::metadata(source_file) {
                    let generate = source_time.modified().unwrap() > target_time.modified().unwrap();
                    if generate {
                        println!("deps-gen: template is more recent");
                    } else {
                        println!("deps-gen: generated file {}s is up-to-date (target={}s)",
                                 source_time.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                                 target_time.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
                    }
                    generate
                } else {
                    eprintln!("deps-gen: can’t get source file metadata");
                    false
                }
            } else {
                println!("deps-gen: no template file, no generation for now, generated file must be removed or lock updated");
                false
            }
        } else {
            println!("deps-gen: target does not exist");
            true
        }
    }
}
