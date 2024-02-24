
#[cfg(test)]
mod tests {
use crate::deps::{Configuration, TemplateSource};
use crate::generator::Generator;

    #[test]
    fn gen_test() {
        let mut configuration = Configuration::default();
        configuration.template = TemplateSource::Text("here
{{#each dependencies}}name={{name}}
{{/each}}
there".into());
        let output = Generator::generate_output(&configuration);
        assert!(output.contains("name=cargo_metadata"));
    }
}
