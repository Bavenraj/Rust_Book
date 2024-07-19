#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
fn main() {
    use crate::List::{Cons, Nil};
    let b = Box::new(5);
    // box type allow ownership transfer but does implement copy trait even if it was i32
    //let a = b;
    println!("b = {b}");
    //println!("a = {a}");

    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    //let list2 = Cons(1, Cons(2, Cons(3, Nil)));

    println!("{list:?}");
    //println!("{list2:?}");
}
