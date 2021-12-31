# References

- Allow referencing of values in multiple places (see [Ownership](ownership.md))
- Denoted in values as `&someStr`, and type annotations as `s: &String`
- Points to original pointer data (so info can be dropped safely)
- _Borrowing_: creating a reference
- Can not modify a reference! Therefore can only modify a value if you own it
- Can only have one mutable ref at a time (prevents data races)
  - Can create scopes to have multiple refs, just not simultaneously!
  - Can't have mutable ref alongside immutable ref for same reasons
