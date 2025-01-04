## Define a macro with macro_rules

This type of macro matches against patterns and replaces the code with other code.

`vec!` is an example of macro

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

Implementation:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

Result of pattern matching:

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

## Procedural macros
Procedural macro acts more like a function (and is a type of procedure).
Procedural macros accept some code as an input, operate on that code, and produce some code as an output.
There are three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.

**When creating procedural macros, the definitions must reside in their own crate with a special crate type.**

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

### Custom `derive` macro
Derive macros works on structs and enums.

Example for creating my own derive macro is here `macro/hello_macro_use/src`.

### Attribute-like macros

Attribute-like macros are similar to custom derive macros,
but instead of generating code for the derive attribute, they allow you to create new attributes.
They’re also more flexible: derive only works for structs and enums;
attributes can be applied to other items as well, such as functions.
Here’s an example of using an attribute-like macro:
say you have an attribute named route that annotates functions when using a web application framework.

```rust
#[route(GET, "/")]
fn index() {
```

The signature of macro would look like this:
```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Other than that, attribute-like macros work the same way as custom derive macros:
you create a crate with the proc-macro crate type and implement a function that generates the code you want!


### Function-like macros

Function-like macros define macros that look like function calls,
they are flexible, they can take an unknown number of arguments.

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check that it’s syntactically correct.
The sql macro would look like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```
