# Logical Expression Parser
This is a simple logical expression parser written in Rust using pest library.
The program consist of two main part:
1. parse_expr function that takes string as input and transform it into abstract syntax tree (AST) and save in Expr enum format.
2. evaluate method implemented for Expr enum that calculate result of expression

## Program has CLI.
Use command 'cargo run -- -h' to display help message.

## Supported Syntax
The parser supports the following syntax:

True: '1'

False: '0'

NOT: '!'

Logical AND: '&&'

Logical OR: '||'

Implication: '->'

Equivalence: '<->'

XOR: 'xor'

parentheses: '(', ')'

### In the AST output:
Integer - 0 or 1

UnaryNot - !

LogicalOp - consist of {

lhs - left operand

op - name of operation

rhs - right operand

}

## Example of usage:
1) input: 1 || 0

output: 
```Abstract syntax tree: {
LogicalOp {
    lhs: Integer(
        1,
    ),
    op: LogicalOr,
    rhs: Integer(
        0,
    ),
}
Result: true
```
2) input: (0 && 1 -> !1 || 0) && 0

output:

```Abstract syntax tree:
LogicalOp {
    lhs: LogicalOp {
        lhs: LogicalOp {
            lhs: Integer(
                0,
            ),
            op: LogicalAnd,
            rhs: Integer(
                1,
            ),
        },
        op: Implication,
        rhs: LogicalOp {
            lhs: UnaryNot(
                Integer(
                    1,
                ),
            ),
            op: LogicalOr,
            rhs: Integer(
                0,
            ),
        },
    },
    op: LogicalAnd,
    rhs: Integer(
        0,
    ),
}

Result: false
```

## Technical Description

### Grammar
The program defines a grammar using the Pest parser generator library in file grammar.pest .
### Grammar Rules:

binary_digit: Matches a binary digit, either "0" or "1."
bin_op: Matches a binary operator, which can be logical AND, logical OR, implication, equivalence, or XOR.
unary_not: Matches the "!" character, representing logical NOT.
logical_and: Matches the "&&" characters, representing logical AND.
logical_or: Matches the "||" characters, representing logical OR.
implication: Matches the "->" characters, representing implication.
equivalence: Matches the "=" character, representing equivalence.
xor: Matches the "xor" characters, representing XOR.
primary: Matches a primary expression, which can be either a binary digit or an expression enclosed in parentheses.
atom: Matches an atom, which is a combination of an optional logical NOT and a primary expression.
expr: Matches a full expression, consisting of atoms separated by binary operators.
equation: Matches the entire equation, surrounded by the Start of Input (SOI) and End of Input (EOI) markers.
WHITESPACE: Matches whitespace, represented by a single space character or a tab.

### Parsing Process
All program logic located in lib.rs file
The parsing process is performed using the Pratt parser, which is implemented in the pest library. The Pratt parser is a top-down operator precedence parser that can handle different precedence levels and associativity for operators. The lazy_static is used to initialize and store the Pratt parser.

### Operators precedence from lowest lo highest(lowest work first):

1. Logical NOT (!)
2. Logical AND (&&)
3. Logical OR (||)
4. Equivalence (<->), XOR(xor)
5. Implication (->)

## Evaluation
After parsing an expression, the program constructs an abstract syntax tree (AST) representing the logical expression. The AST is defined by the Expr and Op enums, where Expr can represent binary digits, unary NOT, or logical operations, and Op represents the logical operators.

The Expr enum is used to evaluate the truth value of the expression. The evaluate method recursively computes the result by evaluating sub-expressions based on the logical operators and truth values.
