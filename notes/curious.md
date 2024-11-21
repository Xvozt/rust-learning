## Relations between vectors, arrays and slices

Example:
```rust
fn parse_config(args: &[String]) -> (&str, &str) {}

let args: Vec<String> = env::args().collect();

let (query, path) = parse_config(&args);
```

Vec<T> and [T] are closely related:
- A Vec<T> owns its data and provides a growable array.
- A slice ([T]) provides a borrowed, read-only view into a contiguous sequence of elements.
- The compiler automatically converts &Vec<T> to &[T] because the two types are compatible.
- Directly passing Vec<T> to a function expecting a &[T] does not work, as the ownership and borrowing rules are different.

## Error handling

When to Use unwrap_or_else or if let
Use unwrap_or_else When:
- The error type is simple (&'static str, Option<T>).
- You can handle the error inline with a default value or by exiting the program.
Use if let Err(e) When:
- The error type is complex or you need to inspect/log the error.
- You want to perform more detailed or specific error-handling logic.

If let is a shorthand of pattern matching.
This:
```rust
match run(config) {
    Err(e) => {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    Ok(_) => {} // Do nothing on success
}
```
Is equivalent to this:
```rust
if let Err(e) = run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
}
```
