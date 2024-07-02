// pub trait Summary {
//     fn summarizz(&self) -> String;
// }
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarizz(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn notify(item: impl Summary) { // or can be like this ->notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarizz());
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

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.headline)
    }
}

// impl Summary for NewsArticle {
//     fn summarizz(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl Summary for Tweet {
//     fn summarizz(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

//use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let news = NewsArticle {
        headline: String::from("Top 10 Movies"),
        location: String::from( "Kuala Lumpur, Malaysia"),
        author: String::from("Tony"),
        content: String::from("Iron Man"),
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    let pair1 = Pair{
        x: 20,
        y: 21,
    };
    println!("New article available! {}", article.summarizz());


    println!("1 new tweet: {}", tweet.summarizz());
    //println!("1 new news: {:?}", notify(news));
    notify(news);
    notify(tweet);
    returns_summarizable();
    let pair2 = Pair::new(20,30);
    pair1.cmp_display();
    pair2.cmp_display();
}