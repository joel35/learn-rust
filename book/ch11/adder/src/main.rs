use adder;

fn main() {
    let num = adder::add_two(5);
    let greeting = adder::greeting("Bubba");
    println!("{}, {}", num, greeting);
}