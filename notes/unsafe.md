## What u can do with unsafe

To switch to unsafe Rust, use the `unsafe` keyword and then start a new block that holds the unsafe code.
You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers.
Those superpowers include the ability to:
- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of a union

### Dereference a raw pointer

As with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`, respectively.
Here with raw pointers we can bypass ownership rules and use mutable and immutable references at the same time:
```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

```

### Calling the unsafe method or function

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

Also we can create a safe abstraction over unsafe code

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

Calling method using FFI always require unsafe block

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### Access or modify a mutable static variable

Static variable are similar to constants with 1 difference:
- values in static variable have fixed address in memory

Static variable can only store references with `static` lifetime.
Accessing an immutable static variables is safe.

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {HELLO_WORLD}");
}
```

But to access mutable static variable we need an unsafe block

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
```

### Unsafe trait

A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
And the implementation is also should be unsafe.

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

### Access fields of union

The final action that works only with unsafe is accessing fields of a union.
A union is similar to a struct, but only one declared field is used in a particular instance at one time.
Unions are primarily used to interface with unions in C code.
Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.
