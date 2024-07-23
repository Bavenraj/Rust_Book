#[derive(Debug)]
enum UsState {
    Alabama,
    Alasa,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
fn ifvalue_in_cents(coin: Coin) -> u8 {
    if let Coin::Penny = coin { println!("Hello");
        1
    } else if let Coin::Nickel = coin {
        5
    } else if let Coin::Dime = coin {
        10
    } else if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
        25
    } else {
        0 
    }
}

fn main(){
    ///let x = 
    value_in_cents(Coin::Quarter(UsState::Alaska));
    //let y = 
    value_in_cents(Coin::Penny);
    ifvalue_in_cents(Coin::Penny);
    //println!("{x} and {y}");
    let five = Some(5);
    let six = plus_one(Some(90));
    println!("{:?}", six);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => { println!("None");
            None}
        Some(i) => {println!("Some");
        Some(i + 1)}
    }
}
