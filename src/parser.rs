use crate::{
    ast::{Args, Evaluate, EvaluationResult, Node, Root, Scope},
    error::Error,
    lexer::tokenize,
    tokens::{Operator, Token, TokenList},
    Result,
};

type NodeList = Vec<Node>;

pub fn eval(expr: &str, scope: &Scope) -> EvaluationResult {
    parse_tokens(tokenize(expr, scope)?, scope)?.eval()
}

pub fn compile<'a>(expr: &'a str, scope: &'a Scope) -> Result<Root<'a>> {
    parse_tokens(tokenize(expr, scope)?, scope)
}

fn encounter_func(f: String, operands: &mut NodeList) -> Result<()> {
    let mut args = Args::with_capacity(2);

    // ASSUMPTION: at least one argument per function
    args.push_front(
        operands
            .pop()
            .ok_or(Error::NotEnoughFunctionParams(f.clone()))?,
    );

    while let Some(last) = operands.pop() {
        if last.token != Token::Comma {
            operands.push(last);
            break;
        } else {
            args.push_front(
                operands
                    .pop()
                    .ok_or(Error::FunctionSyntaxError(f.clone()))?,
            );
        }
    }

    operands.push(Node {
        token: Token::Function(f),
        args: Some(args),
    });
    Ok(())
}

fn right_paren(
    operators: &mut TokenList,
    operands: &mut NodeList,
) -> Result<(), Error> {
    while let Some(top) = operators.pop() {
        match top {
            Token::LeftParenthesis => match operators.last() {
                Some(Token::Function(_)) => {}
                _ => break,
            },
            Token::Function(f) => encounter_func(f, operands)?,
            Token::Operator(op) => add_operator(op, operands)?,
            _ => {}
        }
    }
    Ok(())
}

fn add_operator(operator: Operator, operands: &mut NodeList) -> Result<()> {
    let (n_operands, required) = (operands.len(), operator.num_operands());
    if n_operands < required {
        return Err(Error::MissingOperands(operator));
    };

    let args = operands.split_off(operands.len() - required);
    operands.push(Node {
        token: Token::Operator(operator),
        args: Some(args.into()),
    });

    Ok(())
}

fn encounter_operator(
    cur_op: Operator,
    operators: &mut TokenList,
    operands: &mut NodeList,
) -> Result<()> {
    while let Some(top) = operators.pop() {
        match top {
            Token::Operator(op) => {
                let is_right_assoc = cur_op.is_right_associative();
                if op > cur_op || (op == cur_op && !is_right_assoc) {
                    add_operator(op, operands)?
                } else {
                    operators.push(Token::Operator(op));
                    break;
                }
            }
            Token::Function(f) => encounter_func(f, operands)?,
            _ => {
                operators.push(top);
                break;
            }
        }
    }

    operators.push(Token::Operator(cur_op));
    Ok(())
}

fn parse_tokens(tokens: TokenList, scope: &Scope) -> Result<Root> {
    let mut operators: TokenList = Vec::new();
    let mut operands: NodeList = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => operands.push(Node {
                token: Token::Number(num),
                args: None,
            }),
            Token::Variable(var) => operands.push(Node {
                token: Token::Variable(var),
                args: None,
            }),
            Token::RightParenthesis => {
                right_paren(&mut operators, &mut operands)?
            }
            Token::LeftParenthesis => operators.push(token),
            Token::Operator(op1) => {
                encounter_operator(op1, &mut operators, &mut operands)?;
            }
            Token::Function(f) => operators.push(Token::Function(f)),
            Token::Comma => operands.push(Node { token, args: None }),
        };
    }

    while let Some(Token::Operator(operator)) = operators.pop() {
        add_operator(operator, &mut operands)?
    }

    match operands.pop() {
        Some(node) => Ok(Root { node, scope }),
        None => Err(Error::EmptyExpression),
    }
}
