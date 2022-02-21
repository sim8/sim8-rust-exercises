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
