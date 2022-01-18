use crate::all::Summary;
use std::fmt::Display;
use std::fmt::Debug;
mod all;

fn main() {
    let tweet = all::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // println!("1 new tweet: {}", tweet.summarize());

    let article = all::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    println!("New article available! {}", article.summarize());
    
    notify(&article);

    notify_1(&article);

    notify_2(&tweet, &article);

    notify_3(&tweet, &tweet);

    println!("New article available! {}", article.summarize());
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_2(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}, {}", item.summarize(), item2.summarize());
}

fn notify_3<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}, {}", item.summarize(), item2.summarize());
}

fn notify_4<T: Summary + Display>(item: &T) {

}

fn some_function<T: Display + Clone, U: Clone + Debug>(item1: T, item2: U) {

}

fn some_function_where<T, U>(item1: &T, item2: &U) -> ()
    where T: Display + Clone,
          U: Clone + Debug
{

}

fn returns_summarize() -> impl Summary {
    all::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}