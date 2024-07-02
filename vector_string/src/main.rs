fn main() {
    
    //let mut s = String::new();
    let s = String::from("hello");
    let data = "initial contents";

    let m = data.to_string();
    println!("{data}");
    let m = "initial contents".to_string();

     println!("{s}");
     println!("{m}");

     let hello = String::from("Hola");
     println!("{hello}");

     let mut s1 = String::from("foo");
     let s2 = "bar";
    
     let s3 = String::from("barabara");
     s1.push_str(s2); // can use + instead.
     let s4 = s1 + s2;
     let s5 = format!("{s2}{s3}");// we can still concat string or str using format! function
     
     println!("s2 is {s2} and s1 is none and s3 is {s3} and s4 is {s4} and s5 is {s5}");
     let hello = "Здравствуйте";
     let answer = &hello.len();
     println!("{answer}");
     let hello = "hallo";
    let answer = &hello[0..1];// cannot index by integer but only using range. [0] => [0..1]
     println!("{answer}");

     for c in "Зд".chars() {
        println!("{c}");
    }
}
