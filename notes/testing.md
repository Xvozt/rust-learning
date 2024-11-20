## Useful commands
We are separating arguments to `cargo test` and arguments to the test binary with `--`.

Examples:
```rust
cargo test --help
cargo test -- --help
```

1. To limit threads
```rust
cargo test -- --test-threads=1
```
2. To show output of the under test functions
```rust
cargo test -- --show-output
```

3. To run subset of tests
```rust
cargo test test_function_name_pattern
```

4. To run particular integration test
```rust
cargo test --test integration_test // this will run all the tests in the integration_test.rs file
```

5. To run only ignored
```rust
cargo test -- --ignored
```

6. To

## Organizing tests
1. Unit test live in the files with source code and don't compile not only if you call `cargo test`

```rust
#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_func(){}
}
```
2. We can test private functions in rust, because we are using super::* and items in child module can use items in their ancestor modules
3. Integration tests are living in separete directory and totaly external to the under test library (like any other code that using it) and test only public api

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```
4. To create a module with helper functions for tests we need:
- create separete dir like `common` under `/tests`, this directory won't be treated like test crate
- create `mod.rs` file under that dir
- place functions there

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

```
- declare mod in test with `mod common;` and after we can use functions from that mod in tests

5. We can't create integration tests in binary crates (aka `src/main.rs`), we first need to create lib crate (aka `src/lib.rs`) and move all logic there, and after that create integration tests
