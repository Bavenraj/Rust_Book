//fn main() {
//    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    //let (x, y, z) = _tup;
//    let x = _tup.0;
    //println!("{x}, {y}, {z}")
//    println!("{x}")
//}

use std::io;

fn main() {
    let a = [1, 2, 3,4, 5];
    println!("Please enter and array index");

    let mut index= String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

