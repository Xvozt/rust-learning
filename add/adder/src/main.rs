use add_one;
use add_two;
use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen();
    let random_small: i32 = rng.gen_range(1..10);
    println!(
        "Hello, world! {random_number} plus one is {}!",
        add_one::add_one(random_number)
    );

    println!(
        "Hello world second time! {random_small} plus two is {}",
        add_two::add_two(random_small)
    )
}
