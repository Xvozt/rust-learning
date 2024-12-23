## Main idea

The main difference is "when we know the exat type".
There are two examples:
1. Typical generic type with trait bounds

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Here we know the type at compile type, but in reality
there can be only a vector of one exact type, for example "Button".

2. Trait objects

```rust

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

```

But `Box<dyn Draw>` allow for multiple concrete types to fill for the trait objects in runtime.

## Table with info when to use generics with bounds and trait objects

| Feature           | Generic Type with Trait Bound | Trait Object (dyn Trait)      |
|-------------------|-------------------------------|--------------------------------|
| Polymorphism      | Compile-time (static dispatch)| Runtime (dynamic dispatch)    |
| Performance       | Faster (no vtable lookup)     | Slower (vtable lookup overhead)|
| Binary Size       | Larger (due to monomorphization)| Smaller                       |
| Type Flexibility  | Single concrete type per struct| Multiple types in collections |
| Use Case          | High performance, known types | Flexible, heterogeneous collections |

## Important terms

How i understood them:
1. For generics with boud `monophorization` happens, so compiler generates specific
implementation of generic function for each concrete type
2. Also `static dispatch` happens, so compiler determines which function will be called
using monophorization
3. From the other hand using `trait objects` happens `dynamic dispatch`, when at runtime
using a pointer to function table(vtable) function call is chosen, by doing lookup.
