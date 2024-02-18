use std::fs;
use std::path::PathBuf;

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

impl Default for Configuration {
    fn default() -> Self {
        Self {
            template: TemplateSource::File("src/deps.template.rs".into()),
            cargo_lock_path: "Cargo.lock".into(),
            target_path: None,
            post_template_search: Some("//{}".into()),
            post_template_replace: "".into(),
            include_root: false,
            maximum_depth: None,
        }
    }
}

impl Configuration {
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

