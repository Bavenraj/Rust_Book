enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
   
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
