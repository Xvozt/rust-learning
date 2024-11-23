## Closures syntax

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

## Types are bound after calling closures

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // compiler produce an error because this closure was earlier called with String
```

## Capture values in closures

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:
- borrowing immutably,
```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```
- borrowing mutably,
```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // here we cannot call println!() like above because no more borrows are allowed after first mutable borrows
    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```
- and taking ownership (to do this we are using a special keyword `move`)
```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```
