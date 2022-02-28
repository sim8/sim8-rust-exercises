# Vectors

- `let does_not_exist = &v[100];` // panics if index doesn't exist
- `let does_not_exist = v.get(100);` // returns an optional
- Only stores same type. If need diff types, use an enum with types!
- `String` is a collection of bytes
