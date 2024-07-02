fn main() {

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];

    let third: i32 = v2[2];
    println!("The third element is {third}");
    
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    println!("   ");
    let mut v3 = vec![1, 2, 3, 4, 5];

    let  first = &v3[0];
    println!("The first element is: {first}");
    v3.push(6);
    println!("{:?}", v3);
    //println!("The first element is: {first}");

    let v4: Vec<i32> = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}");
    }
    let mut v5: Vec<i32> = vec![100, 32, 57];
    for i in &mut v5 {
        *i = *i + 50;
        println!("{i}");
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

}
