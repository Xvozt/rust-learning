// the subject of reference doesn't live as long as the reference
// in this case r has a lifetime of a', but refers to to memory with smaller lifetime b'
// this produce compile error from borrow checker

// fn problem_with_lifetime() {
//     let r; // ---------+-- 'a
//            //          |
//     {
//         //          |
//         let x = 5; // -+-- 'b  |
//         r = &x; //  |       |
//     } // -+       |
//       //          |
//     println!("r: {r}"); //          |
// } // ---------+

// here x has a lifetime of b', which is larger than a'. So r can reference x
#[allow(dead_code)]
fn fix_problem_with_lifetime() {
    let x = 5; // ----------+-- 'b
               //           |
    let r = &x; // --+-- 'a  |
                //   |       |
    println!("r: {r}"); //   |       |
                        // --+       |
} // ----------+

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    lifetimes_restrictions_work();
}

// fn longest(x: &str, y: &str) -> &str {
// the problem is that we doesnt know which reference will be returned x or y
// because it depends of the concrete values and the result of if else block
// so we need to delcare relationship of the lifetimes between them using generic annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetimes_restrictions_work() {
    let str1 = String::from("This string is long");

    {
        let str2 = String::from("xyz");
        let res = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {res}");
    }
}

fn lifetimes_restrictions_violated() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
