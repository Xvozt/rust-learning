## When to own or borrow

|               Situation               | Use String | Use &mut String |
|---------------------------------------|------------|-----------------|
| Ownership transfer needed             | ✅          | ❌               |
| Modification without taking ownership | ❌          | ✅               |
| Returning a modified String           | ✅          | ❌               |
| Keeping access to String after call   | ❌          | ✅               |
| Efficient mutation without cloning    | ❌          | ✅               |
| Moving across threads                 | ✅          | ❌               |

## Ownership mode in function signtarures
There are 4 distinct way to pass parameter in a function:

1. ```vec: Vec<i32>``` - pass with immutable ownership transfer of a variable.
- Ownership of vec is transferred to the function, so the caller can no longer use vec after passing it.
- Immutable within the function scope (no mut on the parameter), meaning the function cannot modify vec.
2. ```mut vec: Vec<i32>``` - pass with mutable ownership of a variable.
- Ownership of vec is transferred to the function, and mut allows the function to modify it.
- The caller loses access to vec since ownership is moved.
3. ```vec: &Vec<i32>``` - immutable borrow, we don'move ownership, but borrowing the reference to read the value.
- Borrowing a reference to vec without transferring ownership, so the caller retains access after the function call.
- Immutable borrow, so the function can only read vec but cannot modify it.
- No need for the caller’s variable to be mutable.
4. ```vec: &mut Vec<i32>``` - mutable borrow, we don'move ownership, but borrowing the reference to read and modify value.
- Borrowing a mutable reference to vec without transferring ownership.
- Mutable borrow, so the function can modify vec through the reference.
- Requires that the caller’s variable is mutable to allow a mutable borrow.


## Bindings and lifetimes

Key Takeaway:
Lifetime ≠ Scope:

```rust
fn main() {
    let result;
    {
        let s1 = "literal1"; // `s1` is valid only within this block
        let s2 = "literal2"; // Same for `s2`
        result = get_longest(s1, s2); // `result` points to a 'static str
    } // `s1` and `s2` go out of scope here

    println!("result: {}", result); // ✅ Works because `result` points to a 'static str
    println!("s1: {}", s1);         // ❌ Error: `s1` is out of scope
}
```

The 'static lifetime of a value ensures the value is valid for the program's duration.
The scope of a variable binding controls where you can use the variable name to refer to the value.
Static Lifetime + Out-of-Scope Binding:

Even if a value has a 'static lifetime, if the binding (s1) is declared in a block, you cannot use it outside the block.
