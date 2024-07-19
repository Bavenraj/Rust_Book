use std::ops::Deref;

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
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    use crate::List::{Cons, Nil};
    let b = Box::new(5);
    
    // box type allow ownership transfer but does implement copy trait even if it was i32
    let a = *b.deref();
    println!("b = {b}");
    println!("a = {a}");

    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    println!("{list:?}");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    //MyBox type has no deref trait
    assert_eq!(5, *y);
    println!("x is {x} and y is {}", *(y));

    //deref method return reference and need to be dereference
    assert_eq!(*(y.deref()), *y.deref());

    let a1 = 6;
    let a2 = &a1;
    //let a3 = *a2.deref();
    println!("{:p}, {}, {}", a2, a1, a2.deref());

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

}
