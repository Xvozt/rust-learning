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
    let s2 = s1;
    // let s2 = s1.clone();

    // println!("{s1}, world!"); // won't compile: we cannot borrow value after it moved from s1 to s2.
    println!("{s2}, world!");
}

fn shallow_and_deep_copy() {
    let a = 5;
    let mut b = a;

    println!("a: {a}, b: {b}");

    b = 6;

    println!("a: {a}, b: {b}");
}

fn multiple_returns_calculate_string_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
