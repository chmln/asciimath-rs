use lexer::tokenize;

use ast::{Args, Evaluate, EvaluationResult, Node, Root, Scope};
use error::Error;
use tokens::{Function, Operator, Token, TokenList};

type NodeList = Vec<Node>;

pub fn eval<'a>(expr: &'a str, scope: &'a Scope) -> EvaluationResult {
    parse_tokens(tokenize(expr, scope)?, scope)?.eval()
}

pub fn compile<'a>(expr: &'a str, scope: &'a Scope) -> Result<Root<'a>, Error> {
    parse_tokens(tokenize(expr, scope)?, scope)
}

fn make_node(token: Token, args: Option<Args>) -> Node {
    Node { token, args }
}

fn encounter_func(f: Function, operands: &mut NodeList) -> Result<(), Error> {
    let mut args = Args::with_capacity(2);

    // ASSUMPTION: at least one argument per function
    args.push_front(operands
        .pop()
        .ok_or_else(|| Error::NotEnoughFunctionParams(f.name.clone()))?);

    while let Some(last) = operands.pop() {
        if last.token != Token::Comma {
            operands.push(last);
            break;
        }
        else {
            args.push_front(operands
                .pop()
                .ok_or_else(|| Error::FunctionSyntaxError(f.name.clone()))?);
        }
    }

    operands.push(make_node(Token::Function(f), Some(args)));
    Ok(())
}

fn right_paren(
    operators: &mut TokenList,
    operands: &mut NodeList,
) -> Result<(), Error> {
    while let Some(top) = operators.pop() {
        match top {
            Token::LeftParenthesis => break,
            Token::Function(f) => encounter_func(f, operands)?,
            Token::Operator(op) => add_operator(op, operands)?,
            _ => {},
        }
    }
    Ok(())
}

fn add_operator(
    operator: Operator,
    operands: &mut NodeList,
) -> Result<(), Error> {
    let rhs = operands
        .pop()
        .ok_or_else(|| Error::MissingOperands(format!("{:?}", operator)))?;
    let lhs = operands
        .pop()
        .ok_or_else(|| Error::MissingOperands(format!("{:?}", operator)))?;
    Ok(operands.push(make_node(
        Token::Operator(operator),
        Some(vec_deque![lhs, rhs]),
    )))
}

fn encounter_operator(
    cur_operator: Operator,
    operators: &mut TokenList,
    operands: &mut NodeList,
) -> Result<(), Error> {
    while let Some(top) = operators.pop() {
        match top {
            Token::Operator(top_operator) => {
                if top_operator > cur_operator
                    || (top_operator == cur_operator
                        && !cur_operator.is_right_associative())
                {
                    add_operator(top_operator, operands)?
                }
                else {
                    operators.push(Token::Operator(top_operator));
                    break;
                }
            },
            Token::Function(f) => encounter_func(f, operands)?,
            _ => {
                operators.push(top);
                break;
            },
        }
    }

    debug!("Push op to stack: {:?}", cur_operator);
    operators.push(Token::Operator(cur_operator));
    Ok(())
}

fn parse_tokens<'a>(
    tokens: TokenList,
    scope: &'a Scope,
) -> Result<Root<'a>, Error> {
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
                right_paren(&mut operators, &mut operands)?;
            },
            Token::LeftParenthesis => operators.push(token),

            Token::Operator(op1) => {
                encounter_operator(op1, &mut operators, &mut operands)?;
            },

            Token::Function(f) => operators.push(Token::Function(f)),
            Token::Comma => operands.push(make_node(token, None)),
        };
        debug!("stack: {:?}", operators);
        debug!("output: {:?}", operands);
        debug!("----------");
    }

    while let Some(Token::Operator(operator)) = operators.pop() {
        add_operator(operator, &mut operands)?
    }

    // TODO: revisit this when the final output can also be a string
    operands.pop().map_or_else(
        || Err(Error::EmptyExpression),
        |node| Ok(Root { node, scope }),
    )
}
