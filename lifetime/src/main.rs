#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str, 
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T:Display> (x: &'a str, y: &'a str, ann: T,) -> &'a str{ // since T is generic, we have to explicitly specify display method.
    println!("ANNOUNCEMENT! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); 
        println!("The longest string is {result}");
        println!("The longest string is {}", foo(result));
        println!("{:?}",longest_with_an_announcement(string1.as_str(), string2.as_str(),  "Hellowowowo"));

    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(',').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i.announce_and_return_part("testing"));
   // println!("{:?}",longest_with_an_announcement(string1, string, ann));
    println!("{:?}",i.level());
    println!("{:?}", i.part); //dbg!(i); is similar to println!("{}i:#?}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {// 'a to ensure the variable is valid for the entire function. 
    let x1: &'a str;
    if x.len() > y.len() {
        x1 = x;
    } else {
       x1 =  y;
    }
    x1
}

fn foo(x: &str) -> &str {
    x
}

