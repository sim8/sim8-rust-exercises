# Modules packages crates

- When you `cargo new`, you create a _project_ (defined by cargo.toml)
- A project can contain multiple binaries, and one library, defined by `src/main.rs` and `src/lib.rs` respectively
- Modules defined with `mod { }`. Just a way to organize code. Think filesystem!
- Module paths can be absolute or relative:
  - `crate::front_of_house::hosting::add_to_waitlist();` // absolute
  - `front_of_house::hosting::add_to_waitlist();` // relative
- Add `pub` keyword in front of anything in a module to make it public (usable outside that module)
