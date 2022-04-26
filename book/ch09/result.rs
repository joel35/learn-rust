#![allow(unused)]
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // use_match();
    // use_closure();
    use_unwrap();
    // use_except();
    // dbg!(propagate_error());
    // dbg!(propagate_error_builtin());
    // dbg!(propagate_error_method_chain());
    // dbg!(propagate_error_super_short());
    dbg!(last_char_of_first_line("Hello world!"));

    let f = File::open("hello.txt")?;
    Ok(())
}

fn use_match() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error)
            },
        },
    };
}

fn use_closure() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:>}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}

fn use_unwrap() {
    let f = File::open("hello.txt").unwrap(); // panic if open returns an Err else return the body of Result
}

fn use_except() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagate_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propagate_error_builtin() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? will automatically return a Result if is Err.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn propagate_error_method_chain() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // each ? will exit func if Err else return Ok result

    Ok(s)
}

fn propagate_error_super_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines() // returns iterator of all lines in string
        .next()? // returns first line if there is one or else none (due to the ?)
        .chars() // returns iterator of all the chars of the string slice
        .last() // returns an Option for the last element of the string slice.
}