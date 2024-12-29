fn main() {
    // need to make mut string
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    // to pass it to function change
    change(&mut s1);
    println!("string after change: {s1}");

    multiple_mutable_borrows();
    mutable_and_immutable_borrows_combined();
    mutable_and_immutable_borrows_combined_with_different_scopes();

    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);
    let reference_to_something = no_ref_no_dangle();
    println!("{}", reference_to_something);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(s: &mut String) {
    // because push_str need &mut String to change it with borrowing
    s.push_str(", world!");
}

// very important rule: if you have a mutable reference to a value, you can have no other references to that value.
fn multiple_mutable_borrows() {
    let mut str = String::from("hi");

    let r1 = &mut str;
    // let r2 = &mut str; // that's the problem which will lead to compile error

    // println!("{}, {}", r1, r2);
    println!("{}", r1)
}

// we also cannot combine mutable and immutable references on the same value

fn mutable_and_immutable_borrows_combined() {
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);
}

// but we can "combine" mutable and immutable references on the same value in the different scopes

fn mutable_and_immutable_borrows_combined_with_different_scopes() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem of creating mutable reference
    println!("{}", r3);
    r3.push_str("world!"); // and changing it after
    println!("{}", r3);
}

// dangling reference is a reference to a location in memory that may have been given to someone else
// happens when you freed some memory but there is a pointer to that memory
// this cannot happen in rust

// fn dangle() -> &String {
//     // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// to fix this issue we can return not reference but String itself ownership will move, so nothing will be deallocated
fn no_ref_no_dangle() -> String {
    let s = String::from("no dangle");
    s
}
