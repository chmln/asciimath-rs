use lexer::tokenize;
use std::collections::VecDeque;

use ast::{FunctionArgs, Node};
use tokens::{Function, Operator, Token, TokenList};

type NodeList = Vec<Node>;

pub fn parse(expr: &str) -> Result<Node, String> {
    parse_tokens(tokenize(expr))
}

fn make_node(token: Token, args: Option<VecDeque<Node>>) -> Node {
    Node { token, args }
}

fn encounter_func(f: Function, operands: &mut NodeList) {
    let mut args: FunctionArgs = VecDeque::with_capacity(2);

    // ASSUMPTION: at least one argument per function
    args.push_front(operands.pop().unwrap());

    while let Some(last) = operands.pop() {
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

pub fn right_paren(operators: &mut TokenList, operands: &mut NodeList) {
    while let Some(top) = operators.pop() {
        match top {
            Token::LeftParenthesis => break,
            Token::Function(f) => encounter_func(f, operands),
            Token::Operator(op) => add_operator(op, operands),
            _ => {},
        }
    }
}

pub fn add_operator(operator: Operator, operands: &mut NodeList) {
    let rhs = operands.pop().expect("missing operand");
    let lhs = operands.pop().expect("missing operand");
    operands.push(make_node(
        Token::Operator(operator),
        Some(vec_deque![lhs, rhs]),
    ));
}

pub fn encounter_operator(
    cur_operator: Operator,
    operators: &mut TokenList,
    operands: &mut NodeList,
) {
    while let Some(top) = operators.pop() {
        match top {
            Token::Operator(top_operator) => {
                if top_operator > cur_operator
                    || (top_operator == cur_operator
                        && !cur_operator.is_right_associative())
                {
                    add_operator(top_operator, operands)
                }
                else {
                    operators.push(Token::Operator(top_operator));
                    break;
                }
            },
            Token::Function(f) => encounter_func(f, operands),
            _ => {
                operators.push(top);
                break;
            },
        }
    }

    debug!("Push op to stack: {:?}", cur_operator);
    operators.push(Token::Operator(cur_operator));
}

pub fn parse_tokens(tokens: TokenList) -> Result<Node, String> {
    let mut operators: TokenList = Vec::new();
    let mut operands: NodeList = Vec::new();

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
            Token::RightParenthesis => {
                right_paren(&mut operators, &mut operands)
            },
            Token::LeftParenthesis => operators.push(token),

            Token::Operator(op1) => {
                encounter_operator(op1, &mut operators, &mut operands)
            },

            Token::Function(f) => operators.push(Token::Function(f)),
            Token::Comma => operands.push(make_node(token, None)),
        };
        debug!("stack: {:?}", operators);
        debug!("output: {:?}", operands);
        debug!("----------");
    }

    while let Some(Token::Operator(operator)) = operators.pop() {
        // ASSUMPTION: two operands per operator
        add_operator(operator, &mut operands)
    }

    // TODO: revisit this when the final output is a string/whatever, not just
    // a float
    Ok(operands.pop().unwrap())
}
