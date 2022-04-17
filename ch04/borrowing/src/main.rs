fn main() {

    reference_variable();
    mutable_reference();
    immutable_and_mutable();

}


fn reference_variable() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}


fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here s goes ot of scope. Since it doesn't have ownership of what
  // it refers to, nothing happens.


fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn immutable_and_mutable() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}