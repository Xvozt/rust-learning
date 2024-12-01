use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let b = Box::new("Hello world!");
    println!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // -- deref using
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // -- drop using
    let c = CustomSmartPointer {
        data: String::from("my text"),
    };

    let d = CustomSmartPointer {
        data: String::from("another data"),
    };

    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main function with call of mem::drop function");
    // where variables go out of scope the drop trait will be executed

    // -- Rc<T> using
    // we cannot use moved values
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // But with Rc pointer we can
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
