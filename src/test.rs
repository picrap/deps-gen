
#[cfg(test)]
mod tests {
    use crate::configuration::{TemplateSource};
    use crate::generator::Generator;

    pub type Configuration = crate::configuration::Configuration;

    #[test]
    fn gen_test() {
        let mut configuration = Configuration::default();
        configuration.template = TemplateSource::Text("here
{{#each dependencies}}name={{name}}
{{/each}}
there".into());
        let output = Generator::generate_output(&configuration);
        assert!(output.contains("name=cargo-lock"));
    }
}
