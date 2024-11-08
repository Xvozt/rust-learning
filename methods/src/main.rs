#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(dimension: u32) -> Self {
        Self {
            width: dimension,
            height: dimension,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The rectangle is {rect1:#?}");
    println!("The area of a rectangle is {} square pixels", rect1.area());

    let scale = 3;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 5,
    };

    let rect4 = Rectangle::square(10);

    dbg!(&rect2);

    println!(
        "First {rect1:?} can hold second {rect2:?}: {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "First {rect1:?} can hold second {rect3:?}: {}",
        rect1.can_hold(&rect3)
    );

    dbg!(&rect4);
}
