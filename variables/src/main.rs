fn main() {
    const SECONDS_IN_AN_HOUR: u32 = 60 * 60;

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!(
            "Seconds in inner scope {x} hours: {}",
            x * SECONDS_IN_AN_HOUR
        );
    }

    println!("Seconds in {x} hours: {}", x * SECONDS_IN_AN_HOUR);
}
