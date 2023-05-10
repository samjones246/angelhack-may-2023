use std::{env, process::exit};
use std::fs;
use crate::q1::q1;
use crate::q2::q2;
use crate::q3::q3;
use crate::q4::q4;

mod q1;
mod q2;
mod q3;
mod q4;

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
                Ok(inp) => Ok(q1(&inp)),
                Err(e) => Err(e.to_string())
            }
        },
        2 => {
            match fs::read_to_string("res/q2_input.txt") {
                Ok(inp) => match q2(&inp, 1234) {
                    Ok(x) => Ok(x),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e.to_string())
            }
        },
        3 => Ok(q3(1867)),
        4 => {
            match fs::read_to_string("res/q4_input.txt") {
                Ok(inp) => match q4(&inp) {
                    Ok(x) => Ok(x),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e.to_string())
            }
        }
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
