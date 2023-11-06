use logical_parser_pest::*;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap().trim().to_string();
        let parse_result = CalculatorParser::parse(Rule::equation, &line);
        match parse_result {
            Ok(mut pairs) => {
                let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
                let result = parsed_expr.evaluate();
                println!("Parsed: {:#?}\nResult: {}", parsed_expr, result);
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}
