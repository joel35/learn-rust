mod remove_duplication;
mod generic_types;
mod lifetimes;

use aggregator::{self, NewsArticle, Summary, Tweet};


fn main() {
    play_with_remove_dupe();
    println!();
    play_with_generic_types();
    println!();
    play_with_traits();
    println!();
    println!("*** LIFETIMES ***");
    play_with_lifetimes();
}

fn play_with_remove_dupe() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = remove_duplication::largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = remove_duplication::largest(&number_list);
    println!("The largest number is {}", result);
}

fn play_with_generic_types() {
    let f = generic_types::Point { x: 5.0, y: 11.1 };
    dbg!(f.distance_from_origin());

    let p1 = generic_types::Point { x: 5, y: 10.4 };
    let p2 = generic_types::Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn play_with_traits() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    aggregator::notify(&tweet);
    aggregator::notify(&article);

    let _tweet = returns_summarizable();
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn play_with_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = lifetimes::longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = lifetimes::ImportantExcerpt {
        part: first_sentence,
    };
    dbg!(i);

    //
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = lifetimes::longest_with_an_announcement(string1.as_str(), string2, "Hello world!");
    println!("The longest string is {}", result);



}