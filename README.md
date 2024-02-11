# deps-gen
*From `Cargo.lock` to a generated file.*

# How to use
In `Cargo.toml`, add the following line:
```toml
[build-dependencies]
deps-gen
```
then in your `build.rs`:
```rust
mod deps;

fn main() {
    deps::gen();
    // // or
    // let mut configuration = deps::Configuration::new()
    // // do some changes to configuration here
    // deps::gen_with_configuration(configuration);
}
```
