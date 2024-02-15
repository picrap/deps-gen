
#[cfg(test)]
mod tests {
    use crate::deps;
    use crate::deps::{Configuration, TemplateSource};

    #[test]
    fn gen_test() {
        let mut configuration = Configuration::default();
        configuration.template = TemplateSource::Text("here
{{#each dependencies}}name={{name}}
{{/each}}
there".into());
        let output = deps::debug_generate_output(&configuration);
        assert!(output.contains("name=cargo-lock"));
    }
}
