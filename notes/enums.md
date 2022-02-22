# Enums

- Can have associated data!

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

// Passing a string to initialize object
let loopback = IpAddr::V6(String::from("::1"));
```

## Match

- Use any var as a catch-all, e.g. `other`
- Use `_` as the pattern match if you don't want to use the value
- Return the _unit type_ (`()`) if you don't want to execute anything

## `if let`

- Syntactic sugar for match where you only want to execute for one match
```rust
if let SomeEnum::enumVal(_) = someVar {
    println!("The maximum is configured to be {}", "max");
}
```
