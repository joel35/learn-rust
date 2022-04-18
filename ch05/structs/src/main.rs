fn main() {
    play_with_structs();
    play_with_tuple_structs();
    play_with_unit_like_structs();

}


fn play_with_structs() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}, {}", user2.email, user2.username)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn play_with_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;

fn play_with_unit_like_structs() {
    let subject = AlwaysEqual;
}