#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn methods_consume_iterator() {
        let v = vec![1, 2, 3];

        let iter = v.iter();

        let sum: i32 = iter.sum(); //takes ownership of iter

        assert_eq!(sum, 6);
    }

    #[test]
    fn methods_produce_iterator() {
        let v: Vec<i32> = vec![1, 2, 3];

        // let v2: Vec<_> = v.iter().map(|x| x + 1) // this will produce warning, because iterators are lazy and we are not consuming the iterator

        let v2: Vec<_> = v.iter().map(|x| x + 1).collect(); // if

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 41,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 42,
                style: String::from("sandal"),
            },
            Shoe {
                size: 41,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 41);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 41,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 41,
                    style: String::from("boot"),
                },
            ]
        )
    }
}
