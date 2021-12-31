# Control flow

- if/else
  - expression must be boolean
  - is an expression! So can do inline a la ternary: `let number = if condition { 5 } else { 6 };` (must be same type)
- loops
  - Basic loop: `loop {}` (requires manual `break`)
  - Are expressions! Can use `break 4` to return 4
  - `while` - what it says on the tin
  - `for` - loops items in collection: `for element in arr { }`
    - can run set number of times with range literal, e.g. `for number in (1..4)`
