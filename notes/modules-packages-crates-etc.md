# Modules packages crates

- When you `cargo new`, you create a _project_ (defined by cargo.toml)
- A project can contain multiple binaries, and one library, defined by `src/main.rs` and `src/lib.rs` respectively
- The top-level crate is an implicit module
- Modules defined with `mod { }`. Just a way to organize code. Think filesystem!
- Module paths can be absolute or relative:
  - `crate::front_of_house::hosting::add_to_waitlist();` // absolute
  - `front_of_house::hosting::add_to_waitlist();` // relative
    - `super` == `../`
    - `self` == `./`
- Add `pub` keyword in front of anything in a module to make it public (usable outside that module)
  - If you define a struct as `pub`, the fields are private unless otherwise specified
  - Not true for an enum ^
- Bring stuff in with e.g. `use crate::front_of_house::hosting;`
  - Can use relative paths!
  - By convention, bring in the parent module of whatever you want to use for functions
  - By convention, bring in the exact thing for structs + enums
  - Use `as` to rename imports
  - Can re-export stuff with `pub use`. Handy for libs
  - Bring in multiple things from a thing with `use std::{cmp::Ordering, io};`
  - glob operator is a thing to import everything
