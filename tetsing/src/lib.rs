pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hola {name}")
    // String::from("Hello")
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}
#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 1 {
            panic!("Guess value must be less or equal 100, got value: {value}")
        } else if value < 100 {
            panic!("Guess value must be greater than 1, got value: {value}")
        }

        Guess { value }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn two_plus_two_using_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("Two plus two doesn't equal 4, we got {result}"))
        }
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("We made this test fail");
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Sergio");
        assert!(
            result.contains("Sergio"),
            "Greeting doesn't contain name, value was {result}"
        );
    }

    #[test]
    #[should_panic(expected = "less or equal 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn larger_can_fit_smaller() {
        //arrange
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        //assert
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_fit_larger() {
        //arrange
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        // assert
        assert!(!smaller.can_hold(&larger));
    }
}
