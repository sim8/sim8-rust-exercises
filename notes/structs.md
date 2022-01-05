# Structs

- Struct - defines a shape/types of data
- Object - instance of a struct

```rust
// struct
struct User {
    active: bool, // field
    username: String,
    email: String,
    sign_in_count: u64,
}

// instantiation
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

- Can only set fields using dot notation for mutables
- Update syntax - similar to JS spread BUT with a few major differences
  - 2 dots, not 3
  - Ownership rules apply - fields are either copied or moved over according to datatype rules, that struct may no longer be valid
  - Only uses values not already specified
- Tuple struct - basically a named tuple, e.g. ` struct Color(i32, i32, i32);`

## Methods

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // not a method as missing `self`
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

- Methods receive `self` as first argument
- Defined within an `impl` (implementation block). Everything defined here relates to Rectangle type
  - All fns defined in here are _associated functions_
  - Can have multiple `impl` blocks
- self is shorthand for `self: &Self`, where `self` is an alias for type stated in `impl` block
- Methods can have same name as fields
- Anything without `&self` is not a method. Common uses here are contructors, e.g. `String::from`
