fn main() {
    get_string_slice();
}

fn get_string_slice() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}