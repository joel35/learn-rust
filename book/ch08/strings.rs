fn main() {
    // create empty String
    let mut s = String::new();

    // create String with data
    let data = "initial contents";
    let s = data.to_string(); // option 1
    let s = "initial contents".to_string(); // option 2
    let s = String::from("initial contents"); // option 3

    // can use any utf-8 characters
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // appending to a String
    let mut s = String::from("foo");
    s.push_str("bar");

    // .push_str() takes doesn't take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // add one character
    let mut s = String::from("lo");
    s.push('l');

    // concatenate
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // actually s1.add(&s2) -> s3; .add() will drop s1

    // concatenate with format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // can index Strings with slices (uses bytes so depends on chars stored)
    let hello = "Hello!"; // 1 byte per char
    let s = &hello[0..4]; // first 4 bytes = "Hell"
    println!("{}", s);

    let hello = "Здравствуйте"; // 2 bytes per char
    let s = &hello[0..4]; // first 4 bytes = "Зд"
    println!("{}", s);

    // iterate over strings... specify chars or bytes
    let word = "नमस्ते".to_string(); // 3 bytes per char

    for c in word.chars() {
        println!("{}", c);
    }

    for b in word.bytes() {
        println!("{:#X}", b);
    }

}