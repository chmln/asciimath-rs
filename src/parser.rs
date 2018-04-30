use lexer::tokenize;
use std::collections::VecDeque;

use ast::Node;
use tokens::Token;

pub fn parse(expr: &str) -> Result<Node, String> {
    parse_tokens(tokenize(expr))
}

// TODO: break this down
pub fn parse_tokens(tokens: VecDeque<Token>) -> Result<Node, String> {
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut operand_stack: Vec<Node> = Vec::new();

    let add_node = |token, args| Node {
        token,
        args,
        // lhs: Some(Box::new(lhs)),
        // rhs: Some(Box::new(rhs)),
    };

    for token in tokens {
        println!("TOKEN: {:?}", token);
        match token {
            Token::Number(num) => {
                operand_stack.push(Node {
                    token: Token::Number(num),
                    args: None,
                });
                println!("Add number to output: ");
            }
            Token::Variable(var) => operand_stack.push(Node {
                token: Token::Variable(var),
                args: None,
            }),
            Token::RightParenthesis => {
                while !operator_stack.is_empty() {
                    let top = operator_stack.pop().unwrap();
                    match top {
                        Token::LeftParenthesis => {
                            break;
                        }
                        Token::Function(f) => {
                            let mut args: Vec<Node> = Vec::new();
                            args.push(operand_stack.pop().unwrap());
                            //let last = ;

                            while !operand_stack.is_empty() {
                                let last = operand_stack.pop().unwrap();
                                if last.token != Token::Comma {
                                    operand_stack.push(last);
                                    break;
                                }
                                else {
                                    args.push(operand_stack.pop().unwrap());
                                }
                            }

                            //let lhs = operand_stack.pop().unwrap();
                            //println!("{:?} {:?} {:?}", lhs, f, rhs);
                            operand_stack
                                .push(add_node(Token::Function(f), Some(args)));
                        }
                        _ => {
                            let rhs =
                                operand_stack.pop().expect("missing operand");
                            let lhs =
                                operand_stack.pop().expect("missing operand");
                            operand_stack
                                .push(add_node(top, Some(vec![lhs, rhs])));
                        }
                    }
                }
            }

            Token::LeftParenthesis => {
                operator_stack.push(token);
            }

            Token::Operator(op1) => {
                while !operator_stack.is_empty() {
                    let top = operator_stack.pop().unwrap();

                    match top {
                        Token::Operator(top_operator) => {
                            if top_operator > op1
                                || (top_operator == op1
                                    && !op1.is_right_associative())
                            {
                                let rhs = operand_stack.pop().unwrap();
                                let lhs = operand_stack.pop().unwrap();
                                //println!("{:?} {:?} {:?}", lhs, top_operator, rhs);
                                operand_stack.push(add_node(
                                    Token::Operator(top_operator),
                                    Some(vec![lhs, rhs]),
                                ));
                            }
                            else {
                                operator_stack
                                    .push(Token::Operator(top_operator));
                                break;
                            }
                        }
                        Token::Function(f) => {
                            let mut args: Vec<Node> = Vec::new();
                            args.push(operand_stack.pop().unwrap());
                            //let last = ;

                            while !operand_stack.is_empty() {
                                let last = operand_stack.pop().unwrap();
                                if last.token != Token::Comma {
                                    operand_stack.push(last);
                                    break;
                                }
                                else {
                                    args.push(operand_stack.pop().unwrap());
                                }
                            }

                            //let lhs = operand_stack.pop().unwrap();
                            //println!("{:?} {:?} {:?}", lhs, f, rhs);
                            operand_stack
                                .push(add_node(Token::Function(f), Some(args)));
                        }
                        _ => {
                            operator_stack.push(top);
                            break;
                            //println!("{:?}", top);
                        }
                    }
                }

                println!("Push op to stack: {:?}", op1);
                operator_stack.push(Token::Operator(op1));
            }
            Token::Function(f) => {
                println!("Push func to stack: {:?}", f);
                operator_stack.push(Token::Function(f))
            }
            Token::Comma => {
                operand_stack.push(add_node(token, None));
            }
        };
        println!("stack: {:?}", operator_stack);
        println!("output: {:?}", operand_stack);
        println!("----------");
    }

    while !operator_stack.is_empty() {
        let rhs = operand_stack.pop().unwrap();
        let lhs = operand_stack.pop().unwrap();
        let operator = operator_stack.pop().unwrap();
        operand_stack.push(add_node(operator, Some(vec![lhs, rhs])));
    }

    //println!("{:?}", operand_stack);

    if let Some(node) = operand_stack.pop() {
        //println!("ROOT: {:?}", node);
        Ok(node)
    }
    else {
        Err("empty expression".to_string())
    }
}
