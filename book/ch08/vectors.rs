fn main() {
    // create empty vector
    let a: Vec<i32> = Vec::new();

    // create vector with content
    let b = vec![1, 2, 3];

    // add to pre-existing vector
    let mut c = Vec::new();
    c.push(5);
    c.push(6);
    c.push(7);
    c.push(8);

    { // vector scope
        let d = vec![1, 2, 3, 4];
    } // <- d out of scope and is freed

    // reference vector values
    let e = vec![1, 2, 3, 4, 5];
    // option 1
    let third: &i32 = &e[2];
    println!("The third element is {}", third);
    // option 2
    match e.get(2) { // .get() returns Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // borrowing rules apply
    let mut f = vec![1, 2, 3, 4, 5];
    let first = &f[0]; // immutable borrow
    f.push(6); // mutable borrow, first borrow dropped
    // println!("The first element is: {}", first); // first reference is invalid

    // iterating
    let g = vec![100, 32, 57];
    for i in &g {
        println!("{}", i);
    }

    // mutable iterating
    let mut h = vec![100, 32, 57];
    println!("h: {:?}", h);
    for i in &mut h {
        *i += 50; // * is a dereference operator
    }
    println!("h: {:?}", h);

    // use enum type to wrap different types in a single vector
    // use match to deal with each type. note: must include all types
    // that the vector might get at runtime.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row);
}



