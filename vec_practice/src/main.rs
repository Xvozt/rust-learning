// Given a list of integers,
// use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let mut testvec1 = vec![1, 3, 3, 6, 7, 8, 9];
    let mut testvec3 = vec![5, 3, 3, 3, 2, 2, 1];
    let mut testvec4 = vec![10, 20, 20, 20, 30, 40, 40, 50];
    let mut testvec5 = vec![1, 1, 1, 1];

    // checking median calculation
    assert_eq!(6.0, count_median(&mut testvec1));
    assert_eq!(3.0, count_median(&mut testvec3));
    assert_eq!(25.0, count_median(&mut testvec4));
    assert_eq!(1.0, count_median(&mut testvec5));
    //checking the number with most occurence
    assert_eq!(3, count_element_with_most_occurence(&mut testvec1));
    assert_eq!(3, count_element_with_most_occurence(&mut testvec3));
    assert_eq!(20, count_element_with_most_occurence(&mut testvec4));
    assert_eq!(1, count_element_with_most_occurence(&mut testvec5));
}

fn count_median(v: &mut Vec<i32>) -> f64 {
    v.sort();
    let l = v.len();

    match l % 2 {
        1 => v[l / 2] as f64,
        _others => (v[l / 2 - 1] as f64 + v[l / 2] as f64) / 2.0,
    }
}

fn count_element_with_most_occurence(v: &Vec<i32>) -> i32 {
    let mut occurence_map: HashMap<i32, i32> = HashMap::new();

    for &i in v {
        *occurence_map.entry(i).or_insert(0) += 1;
    }

    *occurence_map
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(k, _v)| k)
        .unwrap()
}
