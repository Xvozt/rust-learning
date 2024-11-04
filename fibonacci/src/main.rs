fn main() {
    println!("{result}", result = fib_gen(0));
    println!("{result}", result = fib_gen(1));
    println!("{result}", result = fib_gen(2));
    println!("{result}", result = fib_gen(3));
    println!("{result}", result = fib_gen(4));
    println!("{result}", result = fib_gen(5));
    println!("{result}", result = fib_gen(10));
    println!("{result}", result = fib_gen(15));
}

fn fib_gen(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut result = 0;
    let mut pre_previous = 0;
    let mut previous = 1;

    for _number in 2..=n {
        result = previous + pre_previous;
        pre_previous = previous;
        previous = result;
    }

    result
}
