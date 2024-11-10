enum Spredsheet {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);
    v.push(7);

    let third = &v[2];
    println!("The third element is {third}");

    let asked = v.get(8);
    match asked {
        Some(asked) => println!("The third element is {asked}"),
        None => println!("There is not such element"),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        print!("{i}, ");
        print!("\n");
    }

    // vec can hold only elements of the same type, but using a Enum we can handle variants with different types in the vec
    #[allow(unused_variables)]
    let row = vec![
        Spredsheet::Int(5),
        Spredsheet::Float(0.0),
        Spredsheet::Text(String::from("Text")),
    ];

    for cell in &row {
        match cell {
            Spredsheet::Int(value) => println!("Integer: {}", value),
            Spredsheet::Float(value) => println!("Float: {}", value),
            Spredsheet::Text(value) => println!("Test: {}", value),
        }
    }

    experiments_with_string();
}

fn experiments_with_string() {
    let mut hello = String::from("Hello");
    hello.push_str(", world!");

    println!("{hello}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s2}");
    println!("{s3}");

    let s1 = String::from("tic-");
    let s2 = String::from("tac");
    // add function is tricky, the signature of it is: fn add(self, s: &str) -> String, so we take ownership of the s1 in this case and doing deref coercion from &String to &str
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    // to work with many strings there is an alternative: format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    // in rust u cannot access a string by index, the code below wont compile
    // Reasons are that you cannot tell what to return: bytes, scalar values or grapheme clusters
    // let s1 = String::from("hello");
    // let h = s1[0];

    // we can iterate over the String using unicode scalars or bytes
    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }
}
