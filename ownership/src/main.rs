fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);
    two_variables_one_string();
    shallow_and_deep_copy();
    let (s, l) = multiple_returns_calculate_string_length(s);
    println!("String: \"{}\" and it's length: {}", s, l);
}

fn two_variables_one_string() {
    let s1 = String::from("hello");
    let s2 = s1; // string doesn't implement copy trait, so the ownership is moved to s2, so that s1 is invalid now
                 // let s2 = s1.clone(); // we need to make deep copy via clone()

    // println!("{s1}, world!"); // won't compile: we cannot borrow value after it moved from s1 to s2.
    println!("{s2}, world!");
}

fn shallow_and_deep_copy() {
    let a = 5; // primitive scalar types implement copy trait
    let b = a;

    let c = (5, false); // tuples too implement copy trait if all values implement copy

    let d = c.0;

    println!("c: {} {}", c.0, c.1);
    println!("a: {a}");
    println!("b: {b}");
    println!("d: {d}");
}

fn multiple_returns_calculate_string_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
