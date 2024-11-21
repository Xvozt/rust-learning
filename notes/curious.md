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
