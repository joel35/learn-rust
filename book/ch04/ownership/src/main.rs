fn main() {
    variable_scope();
    mutable_string();
    move_ownership();
    clone_string();
    copy_stack_data();
    function_ownership();
    return_values_and_scope();
    multiple_returns();

}


fn variable_scope() {
    {  // s is not valid, not yet declared
        let _s = "hello";  // s is valid from here
        // s still valid
    }  // s scope is over, no longer valid
}


fn mutable_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let _s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }   // this scope is now over, and s is no longer valid
}


fn move_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
    println!("{}, world!", s2);
}


fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}


fn copy_stack_data() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}


fn function_ownership() {
    let s = String::from("hello");  // s comes into scope

    println!("original owner: {}", s);

    takes_ownership(s);             // s's value moves into the function...
                                    // ... no longer valid here
    
    let x = 5;                      // x comes into scope
    
    makes_copy(x);                  // i32 always copies into function
                                    // can still reuse here
    
    println!("original: {}", x)
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("new owner: {}", some_string);
} // calls 'drop' on some_string


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("copy: {}", some_integer);
} // some_integer out of scope


fn return_values_and_scope() {
    let s1 = gives_ownership(); // moves its return value into s1
    println!("s1: {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s3 is moved into function
                                       // which moves it back into scope

} // s1 goes out of scope and is dropped
  // s2 was moved so nothing happens
  // s3 goes out of scoppe and is dropped


fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    
    some_string                              // returned and moves out to the
                                             // calling function
}


fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the falling function
}


fn multiple_returns() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}