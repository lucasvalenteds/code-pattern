mod pattern;

use pattern::*;
use std::env;
use std::process::exit;

fn print_help(filename: String) {
    println!("Usage: {} <delimiter> <mask> <raw_code>", filename);
    println!("Example: {} - xx-xx-xxxx 12052005", filename);
}

fn print_error() {
    println!("One or more arguments missing!");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (raw_code, pattern) = match args.len() {
        1 => {
            print_help(args[0].to_string());
            exit(0);
        }
        2 | 3 => {
            print_error();
            exit(1);
        }
        _ => {
            let pattern = Pattern {
                delimiter: String::from(&args[1]),
                mask: String::from(&args[2]),
            };

            let raw_code = String::from(&args[3]);

            (raw_code, pattern)
        }
    };

    match pretty_print(raw_code.to_owned(), pattern) {
        Some(code) => {
            println!("{}", code);
            exit(0);
        }
        None => {
            println!("Could not pretty print the code: {}", raw_code);
            exit(2);
        }
    }
}
