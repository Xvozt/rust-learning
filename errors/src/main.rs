#[allow(unused_imports)]
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

//"long propagation"
fn read_username_from_file(f: &str) -> Result<String, Error> {
    let username_file_result = File::open(f);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//shorter propagation with ?
#[allow(dead_code)]
fn read_username_from_file_other_way(f: &str) -> Result<String, Error> {
    let mut username_file = File::open(f)?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// we can use ? with Option also
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

//propagation with chaining
#[allow(dead_code)]
fn read_username_from_file_with_chaining(f: &str) -> Result<String, Error> {
    let mut username = String::new();

    File::open(f)?.read_to_string(&mut username)?;
    Ok(username)
}

// reading from a file to string using fs which does all the stuff above:
// create a new String, reads the contents of the file, put it into that String and returns it
#[allow(dead_code)]
fn read_username_using_fs(f: &str) -> Result<String, Error> {
    fs::read_to_string(f)
}

fn main() {
    let greeing_file_result = File::open("hello.txt");

    //unwrap to panic straight or return File
    // let greeing_file_result = File::open("hello.txt").unwrap();

    //expect is similar to unwrap but we explicitly give the message to panic
    // let greeing_file_result = File::open("hello.txt")
    //     .expect("Hello.txt should be included in this project");

    #[allow(unused_variables)]
    let greeting_file = match greeing_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem with creating file: {e:?}"),
            },
            other_error => {
                panic!("Problem with opening file: {other_error:?}");
            }
        },
    };

    let username = match read_username_from_file("hello.txt") {
        Ok(name) => name,
        Err(e) => {
            panic!("fail {e:?}")
        }
    };

    println!("{username}");
    let last_char = match last_char_of_first_line(&username) {
        Some(c) => c,
        None => '_',
    };
    println!("{}", last_char)
}
