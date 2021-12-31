# Code basics

- variables immutable by default
- `mut` - sets as mutable
- `//` - comment!
- `::` (e.g. `String::new()`) - associated function of type
- `&` (e.g. `.read_line(&mut guess)`) - indicates variable is a _reference_. Means other parts of code don't have to copy value of
  - references are immutable by default, so need to specify `mut` in above example
- `match` decide what to do next based on output of expression
  - made of _arms_, each of which consists of a _pattern_
  - kind of like a JS switch or if, but guarantees you handle possible cases
- constants must have type annotation
- SHADOWING
  - Allows you to redefine a variable, using `let` again
  - Allows changing type, e.g. useful for parsing something
  - Useful for ensuring a value is subsequently immutable

## Types

- Scalar types (single value): integer, floating point, boolean, char
  - unsigned - will only ever be positive (doesn't need a sign)
  - All int types - i/u 8/16/32/64/128/size (e.g. i16)
  - default int/float: i32, f32
  - char represents unicode character, which might not be a single character (a little confusing)
- Compound types: array, tuple
  - Tuple groups together a fixed-length set of values (may be different types)
    - Define as: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    - Can destructure like so: `let (x, y, z) = tup;`
    - Can access via index, e.g. `tup.2`
  - Array - fixed length list. Must all be same type
    - Define as: `let a = [1, 2, 3, 4, 5];`
    - Data stored on stack
    - Type declaration: `let a: [i32; 5]`
    - Can initialize all values directly if known: `let a = [3; 5];` (sets 5 values to 3)
    - Accessed via index

## Functions

- Hoisting is a thing
- arg type definitions required
- If returns something, we define with arrow + type, e.g. `fn five() -> i32 `
- Can end in expression, which is implicit return (note, expressions omit semicolon)

## SCOPES

- Defined in a curly block, using {}
  More on this later!

## Slices (substring)

- `&someStr[0..2];` - can omit first or last for start/end
- String literals are slices!! They point to specific point of binary. Hence why they're immutable
