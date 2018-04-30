use lexer::tokenize;
use std::collections::VecDeque;

use ast::Node;
use tokens::{Function, Operator, Token};

pub fn parse(expr: &str) -> Result<Node, String> {
    parse_tokens(tokenize(expr))
}

fn make_node(token: Token, args: Option<VecDeque<Node>>) -> Node {
    Node { token, args }
}

fn encounter_func(f: Function, operands: &mut Vec<Node>) {
    let mut args: VecDeque<Node> = VecDeque::with_capacity(2);
    args.push_front(operands.pop().unwrap());

    while !operands.is_empty() {
        let last = operands.pop().unwrap();
        if last.token != Token::Comma {
            operands.push(last);
            break;
        }
        else {
            args.push_front(operands.pop().unwrap());
        }
    }

    operands.push(make_node(Token::Function(f), Some(args)));
}

pub fn right_paren(operators: &mut Vec<Token>, operands: &mut Vec<Node>) {
    while !operators.is_empty() {
        let top = operators.pop().unwrap();
        match top {
            Token::LeftParenthesis => {
                break;
            },
            Token::Function(f) => encounter_func(f, operands),
            Token::Operator(op) => {
                let rhs = operands.pop().expect("missing operand");
                let lhs = operands.pop().expect("missing operand");
                operands.push(make_node(
                    Token::Operator(op),
                    Some(VecDeque::from(vec![lhs, rhs])),
                ));
            },
            _ => {},
        }
    }
}

pub fn encounter_operator(
    op1: Operator,
    operators: &mut Vec<Token>,
    operands: &mut Vec<Node>,
) {
    while !operators.is_empty() {
        let top = operators.pop().unwrap();

        match top {
            Token::Operator(top_op) => if top_op > op1
                || (top_op == op1 && !op1.is_right_associative())
            {
                let rhs = operands.pop().unwrap();
                let lhs = operands.pop().unwrap();

                operands.push(make_node(
                    Token::Operator(top_op),
                    Some(VecDeque::from(vec![lhs, rhs])),
                ));
            }
            else {
                operators.push(Token::Operator(top_op));
                break;
            },
            Token::Function(f) => encounter_func(f, operands),
            _ => {
                operators.push(top);
                break;
            },
        }
    }

    debug!("Push op to stack: {:?}", op1);
    operators.push(Token::Operator(op1));
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Node, String> {
    let mut operators: Vec<Token> = Vec::new();
    let mut operands: Vec<Node> = Vec::new();

    for token in tokens {
        debug!("TOKEN: {:?}", token);
        match token {
            Token::Number(num) => {
                operands.push(Node {
                    token: Token::Number(num),
                    args: None,
                });
                debug!("Add number to output");
            },
            Token::Variable(var) => operands.push(Node {
                token: Token::Variable(var),
                args: None,
            }),
            Token::RightParenthesis =>
                right_paren(&mut operators, &mut operands),
            Token::LeftParenthesis => operators.push(token),

            Token::Operator(op1) =>
                encounter_operator(op1, &mut operators, &mut operands),

            Token::Function(f) => operators.push(Token::Function(f)),
            Token::Comma => operands.push(make_node(token, None)),
        };
        debug!("stack: {:?}", operators);
        debug!("output: {:?}", operands);
        debug!("----------");
    }

    while !operators.is_empty() {
        let rhs = operands.pop().unwrap();
        let lhs = operands.pop().unwrap();
        let operator = operators.pop().unwrap();
        operands.push(make_node(
            operator,
            Some(VecDeque::from(vec![lhs, rhs])),
        ));
    }

    // TODO: revisit this when the final output is a string/whatever, not just
    // a float
    Ok(operands.pop().unwrap())
}
