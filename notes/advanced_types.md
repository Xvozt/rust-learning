## NewType Pattern

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

## Type alias
In this case aliased type is treated like the inner type (i32 in this case).

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);

```

Usually it's useful when working with complex types

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

## The never type

The never type is `!`, it's a return type of a function that never returns. Examples are:
- unwrap func
- match arms

## Dinamically sized types

An example is `str`, but we cannot use it directly, because we don't know how much memory we need to store the value.
So the values of these types should always put behind a pointer. In case of `str` it's `&str` - string slice, which holds 2 values:
- the address of `str`
- the length of `str`

That is, we always know the size of a &str, no matter how long the string it refers to is

Another example of DST (dinamically sized type) is `traits`. For example trait objects should be put behind a pointer
like `&dyn Trait` or `Box<dyn Trait`.
