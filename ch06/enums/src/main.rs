#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn main() {
    do_stuff_with_ips();
    do_stuff_with_message();
    do_stuff_with_option();
}

fn do_stuff_with_ips() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6("::1".to_string());
    dbg!(home, loopback);
}

fn do_stuff_with_message() {
    let m = Message::Write("hello".to_string());
    m.call();
}

fn do_stuff_with_option() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    dbg!(some_number, some_string, absent_number);
}