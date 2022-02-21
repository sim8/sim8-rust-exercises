# Stack / heap

- Stack allocated on a per-thread basis. Space allocated when entering function, space made available when function exists (LIFO)
- Heap allocated at start of process, and removed at end of process.
- Heap slow due to having to follow a pointer, allocation/deallocation overhead, as well as access safety across all threads
- When entering a function (rust) arguments are pushed onto stack

In rust land, data with an unknown size at compile time _must_ live on the heap.
