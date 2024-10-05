use std::{env, process::exit};
use momox::get;

use crate::values::ISBN;

pub mod values;
pub mod momox;

fn main() {
    
    let isbns = match validate_arguments() {
        Ok(isbns) => isbns,
        Err(err) => return handle_error(err),
    };


    let mut prices_momox: Vec<f64> = Vec::new();
    for isbn in isbns {
        let price = match get(isbn) {
            Ok(price) => price,
            Err(err) => return handle_error(err),
        };
        prices_momox.push(price);
    }

    println!("{:#?}", prices_momox)
}

fn handle_error(err: String) {
    println!("{}", format!("Error: {err}"));
    exit(1)
}

fn validate_arguments() -> Result<Vec<ISBN>, String> {
    let args: Vec<String> = env::args().collect();

    let mut counter = 0;
    let mut isbns: Vec<ISBN> = Vec::new();
    for arg in args {
        if counter == 0 {
            println!("Skiping first argument");
            counter += 1;
            continue;
        }

        println!("Handling argument #{:?}", counter);

        let number = match arg.parse::<u64>() {
            Ok(value) => value,
            Err(_) => return Err("Unable to parse value to u64".to_string()),
        };

        let isbn = match ISBN::new(number) {
            Ok(isbn) => isbn,
            Err(err) => return Err(format!("[#{counter}] {err}")),
        };

        isbns.push(isbn);
        counter += 1;
    };

    Ok(isbns)
}

