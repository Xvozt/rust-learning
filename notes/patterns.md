## Refutable and irrefutable
There 2 possible variants:
1. Irrefutable. When a pattern could handle every value, e.g. `let x = 5`, or function parameters `fn foo(x: i32)` or
```rust
let v = vec!['a', 'b', 'c'];
for (index, value) in v.iter().enumerate()
```
2. Refutable. When a pattern can fail to match some possible value it is refutable, example:
```rust
if let Some(color) = favorite_color {
    //some code
}
```

## Corner cases
We cannot use refutable pattern where rust expects irrefutable and vice versa.
Example that won't compile
```rust
 let Some(x) = some_option_value;
 // let expects irrefutable, but Some(x) is refutable
```

we can fix this like:
```rust
 if let Some(x) = some_option_value {
     //some code
 }
 // let expects irrefutable, but Some(x) is refutable
```

another example which gives warning, because it doesn't make sense, but will compile
```rust
if let x = 5 { // this will give warning, because if let is useless, we can use just let
    //some code
}
```

## Match arms
Match arms use refutable pattern, except for the last arm, which uses irrefutable, because should match any remainig values
