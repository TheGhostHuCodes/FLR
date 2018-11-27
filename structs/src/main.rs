struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("example@xyz.com"),
        username: String::from("me"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@abc.com"),
        username: String::from("user2"),
        ..user1
    };

    let user3 = build_user(String::from("user3@xyz.com"), String::from("user3"));

    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{}", user3.email);
}
