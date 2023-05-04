use std::{env, process::exit};
use std::fs;
use angelhack_may_2023::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Must specify question number");
        exit(1);
    }
    let q = match args[1].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("First parameter must be an integer");
            exit(1);
        },
    };
    let result = match q {
        1 => {
            match fs::read_to_string("res/q1_input.txt") {
                Ok(inp) => q1(&inp),
                Err(e) => Err(e)
            }
        },
        _ => { 
            println!("Question not yet implemented");
            exit(1);
        }
    };

    match result {
        Ok(v) => println!("{v}"),
        Err(e) => {
            println!("{e}");
            exit(1);
        }
    };
}
