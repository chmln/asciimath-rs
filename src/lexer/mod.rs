use crate::{
    ast::{resolve_fn, resolve_var, NumericLiteral, Scope},
    error::Error,
    tokens::{Operator, Token, TokenList},
    util::consume_while,
};

fn resolve_vars(expr: &str, scope: &Scope, mut tokens: &mut Vec<Token>) {
    let mut chars = expr.chars();
    let mut var = String::new();
    let mut is_valid_var = false;

    let new_var = |name, t: &mut Vec<Token>| {
        t.push(Token::Variable(name));
        t.push(Token::Operator(Operator::Multiply));
    };

    loop {
        if let Some(c) = chars.next() {
            var.push(c);
            is_valid_var = resolve_var(&var, scope).is_ok();
            if !is_valid_var {
                continue;
            }
        }

        if !var.is_empty() {
            if is_valid_var {
                new_var(var.clone(), &mut tokens);
            } else {
                for c in var.chars() {
                    new_var(c.to_string(), &mut tokens);
                }
            }

            var.clear();
            continue;
        }
        break;
    }
}

fn parse_implicit(
    expr: &str,
    scope: &Scope,
    tokens: &mut TokenList,
) -> Result<(), Error> {
    let mut chars = expr.chars().peekable();

    if tokens.last() == Some(&Token::RightParenthesis) {
        tokens.push(Token::Operator(Operator::Multiply));
    }

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let num = consume_while(chars.by_ref(), |n| {
                    n.is_digit(10) || n == '.'
                });
                let n = num
                    .parse::<NumericLiteral>()
                    .map_err(|_e| Error::InvalidToken(num))?;
                tokens.push(Token::Number(n));
                tokens.push(Token::Operator(Operator::Multiply));
            }
            'a'..='z' | 'A'..='Z' => {
                let vars = consume_while(&mut chars, |c| c.is_alphabetic());
                resolve_vars(&vars, scope, tokens);
                chars.by_ref().next();
            }
            _ => {}
        }
    }
    Ok(())
}

fn get_token(ch: Option<&char>, t: &mut Vec<Token>) -> Option<Token> {
    if let Some(ch) = ch {
        match ch {
            '+' => Some(Token::Operator(Operator::Add)),
            '-' => match t.last() {
                Some(Token::Comma)
                | Some(Token::LeftParenthesis)
                | Some(Token::Function(_))
                | Some(Token::Operator(_))
                | None => {
                    t.push(Token::Number(-1.0));
                    t.push(Token::Operator(Operator::Multiply));
                    None
                }
                _ => Some(Token::Operator(Operator::Substract)),
            },
            '*' => Some(Token::Operator(Operator::Multiply)),
            '/' => Some(Token::Operator(Operator::Divide)),
            '^' => Some(Token::Operator(Operator::Exponentiate)),
            '>' => Some(Token::Operator(Operator::IsGreaterThan)),
            '<' => Some(Token::Operator(Operator::IsLessThan)),
            '=' => match t.last() {
                Some(Token::Operator(Operator::Not)) => {
                    t.pop();
                    Some(Token::Operator(Operator::IsNotEqualTo))
                }
                Some(Token::Operator(Operator::IsGreaterThan)) => {
                    t.pop();
                    Some(Token::Operator(Operator::IsGreaterThanOrEqualTo))
                }
                Some(Token::Operator(Operator::IsLessThan)) => {
                    t.pop();
                    Some(Token::Operator(Operator::IsLessThanOrEqualTo))
                }
                Some(Token::Operator(Operator::IsEqualTo)) => None,
                _ => Some(Token::Operator(Operator::IsEqualTo)),
            },
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            ',' => Some(Token::Comma),
            '!' => Some(Token::Operator(Operator::Not)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn tokenize<'a>(expr: &str, scope: &'a Scope) -> Result<TokenList, Error> {
    let mut chars = expr.chars().peekable();
    let mut tokens = Vec::with_capacity(expr.len());

    while let Some(&_c) = chars.peek() {
        let temp = consume_while(&mut chars.by_ref(), |c| {
            c.is_alphanumeric() || c == '_' || c == '.'
        });

        if !temp.is_empty() {
            if chars.peek() == Some(&'(') && resolve_fn(&temp, scope).is_ok() {
                tokens.push(Token::Function(temp));
                continue;
            } else {
                parse_implicit(&temp, scope, &mut tokens)?;
                if chars.peek() != Some(&'(') {
                    tokens.pop();
                }
            }
        }
        if let Some(token) = get_token(chars.peek(), &mut tokens) {
            tokens.push(token);
        }
        chars.next();
    }

    Ok(tokens)
}
mod tests;
