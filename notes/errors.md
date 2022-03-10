# Errors

- Panic - unrecoverable error, releases memory and exits
- In `cargo.toml` can `[profile.release] \n panic = 'abort'` to have a smaller app that doesn't panic (relies on OS to free memory)
- `RUST_BACKTRACE=1`: env var we can use to see backtraces on panic (debug symbols must be enabled)
