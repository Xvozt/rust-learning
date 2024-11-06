fn main() {
    // let mut s = String::from("Hello world for all rastaceans");
    let s = String::from("Hello world for all rastaceans");
    let s2 = "All the world is opened for rustaceans";
    let word = first_word(&s);
    let word2 = first_word_improved(&s);
    let word3 = first_word_improved(&s[6..]);
    let word5 = first_word_improved(s2);
    let word6 = first_word_improved(&s2[4..]);
    // s.clear(); // clear mutates the string, so need mutable reference
    println!("the word is: {}", word); // println uses reference to word, so the immutable reference to s must still exit, but we cannot combine mutable and immutable reference in the same scope, so there will a compile error on 5:5

    println!("the improved from whole string: {}", word2);
    println!("the improved word from slice of stringt: {}", word3);
    println!("the improved word from string literal: {}", word5);
    println!("the improved word from slice of string literal: {}", word6);

    // Unsized Type (str):

    // The type str by itself is unsized, meaning Rust does not know its exact size at compile time. Because of this, str cannot be passed by itself; it always needs to be referenced through a pointer (like &str or Box<str>).
    // Rust requires knowing the size of types for memory layout, so str can’t exist independently—it needs to be wrapped in a reference or some pointer type.
    // Function Parameter Requirements:

    // In my example, a function with the signature fn some_func(s: str) would expect an unsized value of type str, which isn’t possible in Rust.
    // Instead, the function parameter should be a reference, like &str, so that Rust knows both the address and length of the str data.
}

fn first_word(s: &String) -> &str {
    let str_in_bytes = s.as_bytes();

    for (i, &item) in str_in_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_improved(s: &str) -> &str {
    let str_in_bytes = s.as_bytes();

    for (i, &item) in str_in_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
