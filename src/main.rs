use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use std::io::{self, BufRead};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(implication, Right))
            .op(Op::infix(equivalence, Left) | Op::infix(xor, Left))
            .op(Op::infix(logical_or, Left))
            .op(Op::infix(logical_and, Left))
            .op(Op::prefix(unary_not))
    };
}

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    UnaryNot(Box<Expr>),
    LogicalOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::logical_or => Op::LogicalOr,
                Rule::logical_and => Op::LogicalAnd,
                Rule::implication => Op::Implication,
                Rule::equivalence => Op::Equivalence,
                Rule::xor => Op::Xor,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::LogicalOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_not => Expr::UnaryNot(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}


impl Expr {
    pub fn evaluate(&self) -> bool {
        match self {
            Expr::Integer(val) => *val != 0,
            Expr::UnaryNot(expr) => !expr.evaluate(),
            Expr::LogicalOp { lhs, op, rhs } => {
                let lhs_val = lhs.evaluate();
                let rhs_val = rhs.evaluate();
                match op {
                    Op::LogicalAnd => lhs_val && rhs_val,
                    Op::LogicalOr => lhs_val || rhs_val,
                    Op::Implication => !lhs_val || rhs_val,
                    Op::Equivalence => (lhs_val && rhs_val) || (!lhs_val && !rhs_val),
                    Op::Xor => (lhs_val || rhs_val) && !(lhs_val && rhs_val),
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum Op {
    LogicalAnd,
    LogicalOr,
    Implication,
    Equivalence,
    Xor,
}

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
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
