fn main() {
    let my_first_user = build_user("my_mail@mail.com".to_string(), "Sergio".to_string());
    let black = Color(0, 0, 0);

    println!(
        "
        user with
        email: {}
        name: {}
        registered.
        He is acitve: {}
        and his login count is: {}
        ",
        my_first_user.email, my_first_user.name, my_first_user.active, my_first_user.login_count
    );

    let user2 = User {
        email: String::from("user_2_mail@mail.ru"),
        ..my_first_user // struct update syntax, we just set fields and others are taking from user1
    };

    println!(
        "
        user with
        email: {}
        name: {}
        registered.
        He is acitve: {}
        and his login count is: {}
        ",
        user2.email, user2.name, user2.active, user2.login_count
    );

    println!("Color is: ({},{},{})", black.0, black.1, black.2);
}

struct User {
    active: bool,
    email: String,
    name: String,
    login_count: i32,
}

struct Color(i32, i32, i32);

fn build_user(email: String, name: String) -> User {
    User {
        active: true,
        email,
        name,
        login_count: 1,
    }
}
