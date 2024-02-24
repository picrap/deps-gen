# deps-gen
*From `Cargo.lock` to a generated file.*

# Why?

The goal is to generate a file from a template with dependencies from `Cargo.lock`.

# How?
## Build
In `Cargo.toml`, add the following line:
```toml
[build-dependencies]
deps-gen = "*"
```
then in your `build.rs`:
```rust
use deps_gen;

fn main() {
    gen_deps();
    // // or
    // let mut configuration = deps::Configuration::new()
    // // do some changes to configuration here
    // deps::gen_deps_with_conf(configuration);
}
```

The default configuration will take a `src/deps.template.rs` file and generate a `src/deps.rs` .

## Templating
The library uses handlebars with the default [supported syntax](https://docs.rs/handlebars/5.1.0/handlebars/#built-in-helpers).
The supported field are
- `dependencies`, an array with the following values
  - `name`
  - `version`
  - `authors`
  - `id`
  - `source`
  - `description`
  - `dependencies`
  - `license`
  - `license_file`
  - `targets`
  - `features`
  - `manifest_path`
  - `categories`
  - `keywords`
  - `readme`
  - `repository`
  - `homepage`
  - `documentation`
  - `edition`

see [Rust manifest reference](https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field) for fields details

Also note that `//{}` is replaced with ` ` (an empty string) after template processing. This allows to cleanup.

## An example

`deps.template.rs`:

```rust
#[allow(dead_code)]

pub struct License {
    pub name: &'static str,
    pub version: &'static str,
}

impl License {
    pub fn all() -> Vec<Self> {
        vec![
            //{}{{#each dependencies}}
            Self {
                name: "{{name}}",
                version: "{{version}}",
            },
            //{}{{/each}}
        ]
    }
}
```
