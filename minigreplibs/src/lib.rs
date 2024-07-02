use std::fs;
use std::error::Error;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn run(input1: &String, input2: &String, input3: &String) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(input2)?;
    if input3.to_lowercase() == "yes"{
    for line in search(&input1, &contents) {
        println!("The case sensitive output is executed: {line}");
    }} else{
    for line in search_case_insensitive(&input1.to_lowercase(), &contents) {
        println!("The not case sensitive output is executed: {line}");
    }}

    Ok(())
}
pub fn parse_config(args: &[String]) -> Result<(&String, &String, &String), &str> {
    if args.len() < 4 {
        return Err("not enough arguments");
    }
    let input1 = &args[1];
    let input2 = &args[2]; 
    let input3 = &args[3];
    Ok((input1, input2, input3))
}
