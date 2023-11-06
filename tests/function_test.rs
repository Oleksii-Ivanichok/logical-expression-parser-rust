use logical_expression_parser::*;

#[test]
fn test_parse_expr() {
    let line = 1.to_string();
    let parse_result = CalculatorParser::parse(Rule::equation, &line);
    match parse_result {
        Ok(mut pairs) => {
            let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
            let result = parsed_expr.evaluate();
            assert_eq!(result, true);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
}

#[test]
fn test_parse_expr1() {
    let input = "1 && 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), false);
}

#[test]
fn test_parse_expr2() {
    let input = "0 && 1 -> !1 || 0 && 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), true);
}

#[test]
fn test_parse_expr3() {
    let input = "(0 && 1 -> !1 || 0) && 0";
    let mut pairs = CalculatorParser::parse(Rule::equation, input).expect("Parse failed");
    let parsed_expr = parse_expr(pairs.next().unwrap().into_inner());
    assert_eq!(parsed_expr.evaluate(), false);
}

#[test]
fn test_parse_error_handling() {
    let input = "invalid_input";

    let parse_result = CalculatorParser::parse(Rule::equation, input);

    assert!(parse_result.is_err());
}

#[test]
fn test_parse_expr_complex_expression() {
    let input = CalculatorParser::parse(Rule::expr, "0 && 1 -> !1 || 0 && 0").unwrap();
    let expr = parse_expr(input);

    let expected_expr = Expr::LogicalOp {
        lhs: Box::new(Expr::LogicalOp {
            lhs: Box::new(Expr::Integer(0)),
            op: Op::LogicalAnd,
            rhs: Box::new(Expr::Integer(1)),
        }),
        op: Op::Implication,
        rhs: Box::new(Expr::LogicalOp {
            lhs: Box::new(Expr::UnaryNot(Box::new(Expr::Integer(1)))),
            op: Op::LogicalOr,
            rhs: Box::new(Expr::LogicalOp {
                lhs: Box::new(Expr::Integer(0)),
                op: Op::LogicalAnd,
                rhs: Box::new(Expr::Integer(0)),
            }),
        }),
    };

    assert_eq!(expr, expected_expr);
}
