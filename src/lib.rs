#![allow(non_ascii_idents)]

//!
//! # deps-gen
//! Allows to generate files from `Cargo.lock` and a provided template at build-time
//! ## Example
//! The following will build a file named `src/deps.rs`.
//!
//! In `Cargo.toml`, add the following line:
//! ```toml
//! [build-dependencies]
//! deps-gen = "*"
//! ```
//! then in your `build.rs`:
//! ```rust-notest
//! use deps_gen::gen_deps;
//! 
//! fn main() {
//!     gen_deps();
//! }
//! ```
//! Add `src/deps.template.rs`:
//!
//! ```rust
//! #[allow(dead_code)]
//!
//! pub struct License {
//!     pub name: &'static str,
//!     pub version: &'static str,
//! }
//!
//! impl License {
//!     pub fn all() -> Vec<Self> {
//!         vec![
//!             //{}{{#each dependencies}}
//!             Self {
//!                 name: "{{name}}",
//!                 version: "{{version}}",
//!             },
//!             //{}{{/each}}
//!         ]
//!     }
//! }
//! ```
//! See [readme](https://crates.io/crates/deps-gen) for more details

#[allow(dead_code)]

mod test;
mod generator;
mod data;

use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub enum TemplateSource {
    Text(String),
    File(PathBuf)
}

/// `Configuration` struct
/// - `template` can be either a `File` or `Text`
/// - `cargo_lock_path` is the path to `Cargo.lock` (filled by default)
/// - `target_path` if not specified is deduced for `template` (if `template is specified as `File`) by removing the `.template.` name part.
/// - `post_template_search` / `post_template_replace` allows to perform a post template replacement (to perform cleanup)
/// - `include_root` if the root crate (the one currently being built) has to be included
/// - `maximum_depth` when specified allows to limit recursion (the only interest here is probably `1`)
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
    /// Default values for `Configuration` are
    /// - `template`: TemplateSource::File("src/deps.template.rs".into()), read template from `src/deps.template.rs`
    /// - `cargo_lock_path`: the `Cargo.lock` (how surprising 😅)
    /// - `target_path`: `None` (will be deduced from source path)
    /// - `post_template_search` / `post_template_replace`: `"//{}"` / `""` (meaning `//{}` is removed)
    /// - `include_root`: `false`
    /// - `maximum_depth`: `None`
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

/// For lazy people (like me 😉) the default configuration will take `src/deps.template.rs` to generate `src/deps.rs`
pub fn gen_deps() -> Result<(), Error> {
    gen_deps_with_conf(Configuration::default())
}

/// Detailed generator, allowing to customize configuration
pub fn gen_deps_with_conf(configuration: Configuration) -> Result<(), Error> {
    if let TemplateSource::File(source_path) = &configuration.template {
        println!("cargo:rerun-if-changed={}", fs::canonicalize(source_path)?.to_str().unwrap());
    }
    generator::Generator::gen_with_configuration(configuration)
}

