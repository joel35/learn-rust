use rand::Rng;

fn main() {
    do_stuff_with_match();
    do_more_stuff_with_match();
    play_with_catchall();
    play_with_if_let();

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn do_stuff_with_match() {
    let p = Coin::Penny;
    let value = value_in_cents(p);
    println!("{}", value);

    let value2 = value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn do_more_stuff_with_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(six, none);
}

fn play_with_catchall() {
    let dice_roll = rand::thread_rng().gen_range(1..10);
    println!("Rolled a {}", dice_roll);
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {
    println!("You got a fancy hat!");
}
fn remove_fancy_hat() {
    println!("You lost your fancy hat!");
}
fn reroll() {
    println!("Re-roll!");
    play_with_catchall();
}

fn print_max(max: &Option<u8>) {
    if let Some(i) = max {
        println!("The maximum is configured to be {}", i);
    }
}

fn play_with_if_let() {
    print_max(&Some(3));
    print_max(&Some(6));
    print_max(&None);
}