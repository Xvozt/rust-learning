#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The rectangle is {rect1:#?}");
    println!("The area of a rectangle is {} square pixels", area(&rect1));

    let scale = 3;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };

    dbg!(&rect2);
    // let rect1 = (30, 50);
    // println!("The area of a rectangle is {} square pixels", area(rect1));
    // area using variables
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of a rectangle is {} square pixels",
    //     area(width1, height1)
    // );
}

// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
