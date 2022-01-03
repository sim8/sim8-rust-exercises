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
