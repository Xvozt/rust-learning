fn main() {
    println!("Hello, world!");
    another_func(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {x}");

    {
        let x = plus_one(5);
        println!("The value of x in inner scope is: {x}");
    }
}
fn another_func(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
