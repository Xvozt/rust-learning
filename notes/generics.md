## Lifetime elision rules

The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
the lifetime of self is assigned to all output lifetime parameters.
This third rule makes methods much nicer to read and write because fewer symbols are necessary.
