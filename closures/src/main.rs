use std::thread;
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
#[allow(dead_code)]
impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn giveaway_return_most_stocked_if_no_pref() {
        let store = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref = None;

        assert_eq!(store.giveaway(user_pref), ShirtColor::Red);
    }
    #[test]
    fn giveaway_return_user_pref() {
        let store = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref = Some(ShirtColor::Blue);

        assert_eq!(store.giveaway(user_pref), ShirtColor::Blue);
    }

    #[test]
    fn most_stocked_is_red() {
        let store = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
        };

        assert_eq!(store.most_stocked(), ShirtColor::Red);
    }
}
