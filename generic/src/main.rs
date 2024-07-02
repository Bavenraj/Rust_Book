/*fn largest_i32(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}*/

use std::fmt::{self, Display};
pub struct Human {
    name: String,
    age: u8,
}

impl Human {
    fn walk(&self) {
        println!("Human is walking");
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}


#[derive(Debug)]
struct Point<T> {// can be any letter as long as it is uppercase
    x:T,
    y:T,
}

impl <T: Copy> Point<T>  {//although it is integer, since it is store in generic (T), we have to put T: Copy or only &
    fn x(&self) -> T {
      self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point1<X1, Y1> {
    fn combine<X2, Y2, Y3>(self, second: Point1<X2, Y2>, third: Y3) -> Point1<X1, String>
    where
        Y2: Display,
        Y3: Display,
    {
        Point1 {
            x: self.x,
            y: format!("{}{}", second.y, third),
        }
    }
}

fn main() {
    let number_list = [34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let intee = Point{x:7, y: 8};
    let floatee = Point{x:8.7, y: 6.7};
    let p = Point { x: 5, y: 10 };
  
    println!("{:?}", intee);
    println!("p.x = {}", p.x());
    println!("floatee.x = {}", floatee.distance_from_origin());

    /*let p1 = Point1 {x:5, y: 10.8};
    let p2 = Point1 { x:"Hello", y: 'c'};
    let p3 = Point1 { x: String::from("Hello"), y: 10};
    let p4 = p1.combine(p2, p3);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);*/

    let test = Human{name: String::from("Ali"), age: 22,};
    let p9 = Point1 { x:"jj", y: 'c'};
    let p8 = Point1 { x:"Hello", y: 'c'};
    let py = p9.combine(p8, test);

    println!("py.x = {}, py.y = {}", py.x, py.y);


}

//combined function using generic
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

