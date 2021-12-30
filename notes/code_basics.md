# Code basics

- variables immutable by default
- `mut` - sets as mutable
- `//` - comment!
- `::` (e.g. `String::new()`) - associated function of type
- `&` (e.g. `.read_line(&mut guess)`) - indicates variable is a _reference_. Means other parts of code don't have to copy value of
  - references are immutable by default, so need to specify `mut` in above example
