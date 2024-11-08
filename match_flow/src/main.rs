enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("player got a fancy hat");
}
fn remove_fancy_hat() {
    println!("player lost a fancy hat");
}
fn move_player(num_spaces: u8) {
    println!("player moved for {num_spaces} cells");
}

fn main() {
    let rare_coin = Coin::Quater(UsState::Alaska);
    value_in_cents(rare_coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());
    println!("{}", none.unwrap_or_default());

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        5 => move_player(5),
        _ => (),
    }
}
