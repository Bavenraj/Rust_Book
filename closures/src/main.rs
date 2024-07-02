 use core::time::Duration;
 use std::thread;
 #[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //user_preference.unwrap_or_else(|| self.most_stocked())
        match user_preference {
            Some(color) => color,
            None => self.most_stocked(),
        }
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    let add_number = |x, y, z| x+y+z; // similar to fn add_number(x, y, z) -> u32{ x + y + z };
    let result = add_number(1, 2,3);
    println!("the result of addition is {result}");
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        //thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(20);
    let example_closure = |x| x;
    let s = example_closure(String::from("heloo"));
    println!("{s}");
    // let n = example_closure(5);
    
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");    
    let only_borrows = || println!("From closure: {list:?}");
    
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After calling closure: {list2:?}");

    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {list3:?}");

    thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(sort_by_width);//need additional function if dont want to use closure
    println!("{list:#?}");

    let mut list2 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list2.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width
    });
    println!("{list2:#?}");

}

fn sort_by_width(rect: &Rectangle) -> u32 {
rect.height
}