use std::fmt::Display;

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest is x: {}", self.x)
        } else {
            println!("The largest is y: {}", self.y)
        }
    }
}

// blanket implementation
// we can implement a trait for any type that implements another trait
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        todo!()
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("author: {}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.unsername, self.content)
    // }
}

// example of impl trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//example of verbose syntax (the idea is the same as above)
// pub fn notify<T: Summary>(item: &T) {}

//but the difference is clear when we want to pass more than 1 parameter.
// Both item1 and item2 must be the same type (which, for sure must implement Summary)
// This signature is called trait bounds
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {} \n{}",
        item1.summarize(),
        item2.summarize()
    );
}
// and in this case item1 and item2 could be different types (but still must implement Summary)
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Breaking news! {} \n{}",
        item1.summarize(),
        item2.summarize()
    );
}

// Multiple trait bounds
// pub fn notify_multiple_traits(item: &(impl Summary + Display)) {}
// pub fn notify_multiple_traits<T: Summary + Display>(item: &T) {}

// Multiple trait bounds with multiple types
// istead of this mess
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// we can use where clause

// Fix Diagnostics
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Debug + Clone,
// {
// }
