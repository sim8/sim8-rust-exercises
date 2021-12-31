# Ownership

TL;DR

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Above has some special implications:

- `Drop` trait is used to discard value once goes out of scope
- `str1 = str2` will mean `str1` is no longer valid
  - If a type implements `Copy` trait, above does not apply (not allowed if type also implements `drop`)
  - `Copy` implemented on all scalar types and tuples, provided they only contain scalar types
  - `str.clone()` method is commonly used way to deep copy something
- similarly, passing value to another func will mean it's invalid for subsequent use
  - a workaround for this is returning the result _and_ the original arg (maintaining the value), but rust provides _References_ as the proper solution here

## Stack vs. Heap

- Stack
  - Items must have known, fixed size
- Heap
  - Items can have variable sizes. At time of creation, a space of a specified size is requested from memory allocator, pointer is returned
  - Requesting is done automatically, e.g. `String::from` (works similarly in most langs)
- When copy a point (e.g. `str1 = str2`), you are copying the info held in the stack (address, length, capacity).
