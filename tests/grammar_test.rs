use logical_expression_parser::*;
use pest::Parser;

#[test]
fn test_rule_binary_digit() {
    let input = "0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), false);
}

#[test]
fn test_rule_unary_not() {
    let input = "!0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_rule_logical_and() {
    let input = "1 && 1";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_rule_logical_or() {
    let input = "1 || 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_rule_implication() {
    let input = "1 -> 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), false);
}

#[test]
fn test_rule_equivalence() {
    let input = "1 <-> 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), false);
}

#[test]
fn test_rule_xor() {
    let input = "1 xor 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_rule_primary() {
    let input = "!(!(1)) && 1";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_rule_atom() {
    let input = "!(!(1))";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_rule_expr() {
    let input = "(0 && 1 -> !1 || 0) && 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), false);
}
