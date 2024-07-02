use std::env;
use std::process;
use minigreplibs::{parse_config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (input1, input2, input3) = parse_config(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing: {err}");
        process::exit(1);
    });

    println!("Searching for {input1}");
    println!("In the file {input2}");
    println!("Case sensitive: {input3}");

    if let Err(e) = run(&input1, &input2, &input3){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

