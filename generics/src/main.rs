struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Flex<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Flex<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Flex<X2, Y2>) -> Flex<X1, Y2> {
        Flex {
            x: self.x,
            y: other.y,
        }
    }
}

fn find_largest_in_the_list(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number
        }
    }

    largest
}

fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    //extracted to find_largest_in_the_list
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    let largest = find_largest_in_the_list(&number_list);
    println!("Largest number from the list is: {largest}");

    let number_list = vec![1, 2, 5, 6, 8];
    let result = find_largest_in_the_list(&number_list);
    println!("Largest is: {result}");

    let char_list = vec!['y', 'b', 'a', 'q'];
    let result = largest_generic(&char_list);
    println!("Largest char is: {result}");

    let unsigned_numbers: Vec<u32> = vec![1, 100, 500, 100];
    let largest_among_unsigned = largest_generic(&unsigned_numbers);
    println!("Largest unsigned is: {largest_among_unsigned}");
    let some_new_float_numbers: Vec<f64> = vec![1.1, 2.2, 3.3];
    let largest_among_float = largest_generic(&some_new_float_numbers);
    println!("Largest float is: {largest_among_float}");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y);

    let p_float = Point {
        x: 5.0 as f32,
        y: 7.5 as f32,
    };
    println!("The distance is: {}", p_float.distance_from_origin());

    let f1 = Flex { x: 5, y: 10.4 };
    let f2 = Flex { x: "hello", y: 'a' };

    let f3 = f1.mixup(f2);
    println!("f3.x = {}, f3.y = {}", f3.x, f3.y);
}
