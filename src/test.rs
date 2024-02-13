
#[cfg(test)]
mod tests {
    use crate::deps;
    use crate::deps::Configuration;

    #[test]
    fn gen_test() {
        let mut configuration = Configuration::new();
        configuration.template = Some("here\
{{#each dependencies}}
name={{name}}
{{/each}}
there".into());
        let output = deps::debug_generate_output(&configuration);
        assert!(output.contains("name=cargo-lock"));
    }
}
