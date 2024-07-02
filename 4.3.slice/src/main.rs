// fn main() {
//     let x = String::from("Hello everyone");
//     let y = first_word(&x);
//     let z = second_word(&x);
//     println!("First word: {y}, Second word: {z}");
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     println!("{:?}", bytes);
//     let mut i = 0;
//     for byte in bytes{
//         //println!("{i}");
//         if *byte == b' ' {
//             return i;
//         }
//         i = i + 1;
//     }
//    s[..]
// }

// fn second_word(s: &String) -> usize{
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         //println!("{i}");
//         if item == b' '{// can be replace with 32 as it is also a space character
//             return i;
//         }
//     }
//     s.len()
// }

fn new_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    // {
    //     let my_string = String::from("hello world");
    //     let word = new_word(&my_string[0..6]);
    //     println!("new_word(&my_string[0..6]): {word}");
    //     let word = new_word(&my_string[..]);
    //     println!("new_word(&my_string[..]): {word}");
    //     let word = new_word(&my_string[6..]);
    //     println!("new_word(&my_string): {word}");
    // }
    let my_string_literal = "hello world";
    // let word = new_word(&my_string_literal[0..6]);
    // println!("new_word(&my_string[0..6]): {word}");
    // let word = new_word(&my_string_literal[..]);
    // println!("new_word(&my_string[..]): {word}");
    let word = new_word(&my_string_literal[6..]);
    println!("new_word(&my_string): {word}");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    //let sli = assert_eq!(slice, &[2, 3]);
    println!("{slice:?}");
}
