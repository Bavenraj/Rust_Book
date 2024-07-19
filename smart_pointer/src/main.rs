#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use crate::List::{Cons, Nil};
    let b = Box::new(5);
    println!("b = {b}");

    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    //let list2 = Cons(1, Cons(2, Cons(3, Nil)));

    println!("{list:?}");
    //println!("{list2:?}");
}
