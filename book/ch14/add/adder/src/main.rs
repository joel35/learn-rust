use add_one;
use add_two;
use rand;

fn main() {
    let num:i32 = rand::random::<u8>() as i32;
    println!(
        "Hello, world! {} plus one is {}",
        num,
        add_one::add_one(num)
    );
    println!(
        "Hello, world! {} plus two is {}",
        num,
        add_two::add_two(num)
    );


}
