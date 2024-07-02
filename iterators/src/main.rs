fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    println!("\nBy using v1.iter()");
    for val in v1_iter {
        println!("Got: {val}");
        //println!("{:?}", v1_iter.next());
    }
    println!("\nWithout using v1.iter()");
    for val in v1 {
        println!("Got: {val}");
       // println!("{:?}", v1_iter.next());
    }
    println!("\n");

    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    let v3 = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum();
    assert_eq!(total, 6);

    let v4: Vec<i32> = vec![1,2,3];
    let v5: Vec<_> = v4.iter().map(|x| x + 1).collect();
    assert_eq!(v4, vec![2,3,4]);

}

