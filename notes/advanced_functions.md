## Function pointers

It can be useful to pass function, that was already defined instead of defining a new closure.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}
```

### Returning a closure

We can't do this:
```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

We cannot directly return a closure, because of Sized trait, because the compiler doesn't know how much space we need
to store the closure.

But we can return a trait object:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
