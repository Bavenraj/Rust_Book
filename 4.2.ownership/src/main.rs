// fn main() {
//     let mut s = String::from("hello");
//    // let s = String::from("hello");

//     s.push_str("Testtt");
//     let m = s.clone();
//     println!("{s}");
//     println!("{m}");
// }

// fn main(){
//     let  mut s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{s1}' is '{len}.");
//     let x = change(&mut s1);
//     let y = &s1;
//    // let z = &mut s1;
//     // let mut x = "hello";
//     //  x = concat(x , "ll");
//     println!("{y}");
// }
//  fn calculate_length(s:&String) -> usize {
//     s.len()    
//  }

//  fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// we cannot reference y= "hellp" to x = y but can x =&y
// once we store x into y, x become invalid in string but not in int and str

fn main() {
    let reference_to_nothing = danle();
    println!("{reference_to_nothing}");
}

fn danle() -> String {
    let s = String::from("hello");
    s
}