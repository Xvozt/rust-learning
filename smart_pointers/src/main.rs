use std::cell::RefCell;
use std::{ops::Deref, rc::Rc, rc::Weak};

use crate::CycleList::{CycleCons, CycleNil};
use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RcRfList::{RcRfCons, RcRfNil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RcRfList {
    RcRfCons(Rc<RefCell<i32>>, Rc<RcRfList>),
    RcRfNil,
}
#[derive(Debug)]
enum CycleList {
    CycleCons(i32, RefCell<Rc<CycleList>>),
    CycleNil,
}

impl CycleList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            CycleCons(_, item) => Some(item),
            CycleNil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
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

    // -- RefCell with Rc to have multiple owners AND mutate the value

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcRfCons(Rc::clone(&value), Rc::new(RcRfNil)));
    let b = RcRfCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcRfCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    //--- Reference cycle example here
    println!("Reference cycle example starts here");
    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));

    println!("\"a\" initial rc count = {}", Rc::strong_count(&a));
    println!("\"a\" next item = {:?}", a.tail());

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));

    println!("\"a\" rc count after b creation = {}", Rc::strong_count(&a));
    println!("\"b\" initial rc count = {}", Rc::strong_count(&b));
    println!("\"b\" next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //Weak smart pointers example starts here

    println!("Experiment with Weak downgrade and upgrade starts here");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
