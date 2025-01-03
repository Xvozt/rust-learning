## Associated types

The type Item is a placeholder, and the next method’s definition shows that it will return values of type Option<Self::Item>.
Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Implementation

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--

```

### Why not use generics

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is that when using generics, we must annotate the types in each implementation;
because we can also implement Iterator<String> for Counter or any other type,
we could have multiple implementations of Iterator for Counter.
In other words, when a trait has a generic parameter,
it can be implemented for a type multiple times,
changing the concrete types of the generic type parameters each time.
When we use the next method on Counter,
we would have to provide type annotations to indicate which implementation of Iterator we want to use.

With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.
Also associated type is a part of trait contract: mplementors of the trait must provide a type to stand in for the associated type placeholder.


## Supertraits

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

In this case the type which implements `OutlinePrint` should also implement `Display` trait.

## NewType pattern for implement traits on external types

Let's say we want to implement `Display` trait on a `Vector` type. We cannot do this by default, because both a trait
and a type are external to our crate and orphan rule won't let us do this. But we can create a wrapper using tuple struct.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

The downside is that we must implement all methods of the value that wrapper holds.
Or we could implement `Deref` trait to return the inner type.
