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
        print!("{i}, ")
    }

    // vec can hold only elements of the same type, but using a Enum we can handle variants with different types in the vec
    #[allow(unused_variables)]
    let row = vec![
        Spredsheet::Int(5),
        Spredsheet::Float(0.0),
        Spredsheet::Text(String::from("Text")),
    ];
}
