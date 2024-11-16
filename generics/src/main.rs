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
}
