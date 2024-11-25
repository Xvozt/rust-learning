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

*Move closures may still implement `Fn` and `FnMut` traits even though they captture the variables by move. This is because the tait implemented by a closure type are determined by `what` the closure does with captured values, not `how` it captures them.*

## Fn traits for closures

Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

- `FnOnce` applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
Example:

```rust
fn apply_to_5_once<F>(f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(5)
}
```

Usage:

```rust
fn main() {
    let s = String::from("hello");
    let consume_and_return_length = |x| {
        println!("Consumed string: {}", s);
        x + s.len()
    };
    let result = apply_to_5_once(consume_and_return_length);
    println!("Result: {}", result); // Output: Result: 10
}
```

- `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
Example:

```rust
fn apply_to_5_mut<F>(mut f: F) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(5)
}
```

Usage:

```rust
fn main() {
    let mut total = 0;
    let mut add_to_total = |x| {
        total += x;
        total
    };
    let result = apply_to_5_mut(add_to_total);
    println!("Result: {}", result); // Output: Result: 5
}
```


- `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.
These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
Example:

```rust
fn apply_to_5<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5) // Call the closure with 5 as the argument
}
```

Usage:

```rust
fn main() {
    let add_one = |x| x + 1; // Closure to add 1 to its input
    let result = apply_to_5(add_one);
    println!("Result: {}", result); // Output: Result: 6
}
```

## !TODO i need to investigate closures more, understood it very poorly for the first time, especially with generics and trait bounds

The topic in the book - 13.1

list of actions:
- return to the topic
- find more examples
- ask chatgpt to generate some quizes and ask him questions to deeply understadn the topic
