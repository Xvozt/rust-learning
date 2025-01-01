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


## Pattern syntax


### literals

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### names variables

Variables that declared inside the match scope shadow those that are outside with the same names.
In the example below we use x that is `Some(5)`, because so we arrive at the secon arm, but we dont use y that is 10, we shadow it with the value
from `Some(5)`, so we print `Matched, y = 5`. And finallyt at the last `println` call we print `at the end: x = Some(5), y = 10`.

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {y}"),
    _ => println!("Default case, x = {x:?}"),
}

println!("at the end: x = {x:?}, y = {y}");

```

### multiple patterns

We can match multiple patterns using `|`.

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

```

### matching ranges

But we can simplify match of the multiple patterns with ranges. But it's only possible with numeric types or `char`.

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

```

Or with `char`:

```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}

```

### Destructuring to break apart values

#### Destructuring structs
This is tricky, but we can descrtucture a structure using patterns

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

Or more elegant version.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

Or with literal values. Below we have a match expression that separates Point values into three cases:
points that lie directly on the x axis (which is true when y = 0),
on the y axis (x = 0),
or neither.

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

#### Destructuring enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}
```

#### Destructuring nested structs or enums

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```

#### Complex destructures

We can mix, match, and nest destructuring patterns in even more complex ways.
The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

This code lets us break complex types into their component parts so we can use the values we’re interested in separately.


### Ignore values in a pattern

#### Ignore the entire value

It can be useful when implementing a trait with some certain signature, but you don't need the value

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}
```

#### Ignore parts of value

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {setting_value:?}");

```

#### The difference between `_` and `_s`

The main idea is `_s` will still bind the value to `_s`, but `_` won't

```rust
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{s:?}");

```

In this case the `s` value will still be moved to `_s` so we cannot use s again
But if we just use `_` there won't be any problem:

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{s:?}");

```

#### Ignoring Remaining Parts of a Value with ..

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {x}"),
}
```

The syntax .. will expand to as many values as it needs to be. Listing 18-24 shows how to use .. with a tuple.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

### Extra conditionals in match patterns (`match guard`)

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    Some(x) => println!("The number {x} is odd"),
    None => (),
}

```


#### Pattern-shadowing problem

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}
```

#### `or` operator with match guard

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}

```

### @ binding

The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
In Listing 18-29, we want to test that a Message::Hello id field is within the range 3..=7.
We also want to bind the value to the variable id_variable so we can use it in the code associated with the arm

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {id_variable}"),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {id}"),
}

```
