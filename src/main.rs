use logical_expression_parser::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn print_help() {
    println!("Usage of logical expression parser:");
    println!("Use 'cargo run' to run the program in default mode. Enter a single logical expression in the terminal to parse.");
    println!("You can use the program with options:");
    println!(" 'cargo run -- -h' to display this help message.");
    println!(" 'cargo run -- -c' to display credits information.");
    println!(" 'cargo run -- -f filepath' to parse a single expression from a file.");
    println!(" 'cargo run -- -p \"expression\"' to parse a single expression.");
}

fn print_credits() {
    println!("Logical expression parser - Version 1.0");
    println!("Author: Oleksii Ivanichok");
    println!("License: MIT");
}

fn parse_expr_from_string(expression: &str) -> io::Result<()> {
    let parse_result = CalculatorParser::parse(Rule::equation, expression);
    match parse_result {
        Ok(mut pairs) => {
            let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
            let result = parsed_expr.evaluate();
            println!("Abstract syntax tree:\n{:#?}\nResult: {}", parsed_expr, result);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
    Ok(())
}

fn parse_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        parse_expr_from_string(&line)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        for line in io::stdin().lock().lines() {
            let line = line.unwrap().trim().to_string();
            parse_expr_from_string(&line)?;
        }
    } else {
        match args[1].as_str() {
            "-h" => {
                print_help();
            }
            "-c" => {
                print_credits();
            }
            "-f" if args.len() == 3 => {
                let filename = &args[2];
                parse_file(filename)?;
            }
            "-p" if args.len() == 3 => {
                let expression = &args[2];
                parse_expr_from_string(expression)?;
            }
            _ => {
                eprintln!("Invalid command-line arguments. Use '-h' or '--help' for usage information.");
            }
        }
    }

    Ok(())
}
