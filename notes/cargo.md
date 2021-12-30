# Cargo

- Build system + package manager!
- TOML - Tom's obvious minimal language
- crates - packages in rust land
- `cargo build` - two guesses to what this does :)
  - also generates `cargo.lock` to track dep versions
  - `--release` optimized for prod
- `cargo run` - above plus runs target output
- `cargo check` - checks the code compiles
- default registry is `crates.io`
- `cargo update` - ignores `Cargo.lock` and updates all deps, according to `.toml` config (updating lock file in the process)
- `cargo doc --open` - generates local docs for your deps!
