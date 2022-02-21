# Traits

## Derived traits

### Debug

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    // Derive debug required to use with {:?} - used for printing data structures
}

```

### Copy
- Only available for stack types
- Not available it type already derives `drop`
