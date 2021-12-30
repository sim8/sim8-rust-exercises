# std

- standard library!
- Prelude
  - An auto-imported list of stuff for each program
  - https://doc.rust-lang.org/std/prelude/index.html
- `read_line`: appends terminal input to variable. Variable needs to be mutable
- `Result` - enum, variants are `Ok` + `Err`. Both contain info
  - `.expect()` handles error + crashes program
