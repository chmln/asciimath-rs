use ast::{resolve_fn, resolve_var, NumericLiteral, Scope};
use error::Error;
use std::{iter::Peekable, str};
use tokens::{Function, Number, Operator, Token, TokenList, Variable};

fn consume_while<F>(it: &mut Peekable<str::Chars>, x: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut s = String::with_capacity(5);

    while let Some(&ch) = it.peek() {
        if x(ch) {
            it.next().unwrap();
            s.push(ch);
            continue;
        }
        break;
    }
    s
}

fn resolve_vars(expr: &str, scope: &Scope, mut tokens: &mut Vec<Token>) {
    let mut chars = expr.chars();
    let ref mut var = String::new();
    let mut is_valid_var = false;

    let new_var = |name, t: &mut Vec<Token>| {
        t.push(Token::Variable(Variable { name }));
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
            }
            else {
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

fn parse_implicit(expr: &str, scope: &Scope) -> Result<TokenList, Error> {
    let mut tokens: TokenList = Vec::with_capacity(expr.len() * 2);
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        let mut r = chars.by_ref();
        match ch {
            '0'...'9' => {
                let num = consume_while(r, |n| n.is_digit(10) || n == '.');
                let n = num.parse::<NumericLiteral>()
                    .map_err(|_e| Error::InvalidToken(num))?;
                tokens.push(Token::Number(Number::new(n)));
                tokens.push(Token::Operator(Operator::Multiply));
            },
            'a'...'z' | 'A'...'Z' => {
                let mut vars = consume_while(r, |c| c.is_alphabetic());
                resolve_vars(&vars, scope, &mut tokens);
                r.next();
            },
            _ => {},
        }
    }
    Ok(tokens)
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
                    t.push(Token::Number(Number::new(-1)));
                    t.push(Token::Operator(Operator::Multiply));
                    None
                },
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
                },
                Some(Token::Operator(Operator::IsGreaterThan)) => {
                    t.pop();
                    Some(Token::Operator(Operator::IsGreaterThanOrEqualTo))
                },
                Some(Token::Operator(Operator::IsLessThan)) => {
                    t.pop();
                    Some(Token::Operator(Operator::IsLessThanOrEqualTo))
                },
                Some(Token::Operator(Operator::IsEqualTo)) => None,
                _ => Some(Token::Operator(Operator::IsEqualTo)),
            },
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            ',' => Some(Token::Comma),
            '!' => Some(Token::Operator(Operator::Not)),
            _ => None,
        }
    }
    else {
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
                tokens.push(Token::Function(Function::new(temp.clone())));
                chars.by_ref().next();
                continue;
            }
            else {
                if tokens.last() == Some(&Token::RightParenthesis) {
                    tokens.push(Token::Operator(Operator::Multiply));
                }
                tokens.append(&mut parse_implicit(&temp, scope)?);
                if chars.peek() != Some(&'(') {
                    tokens.pop();
                }
            }
        }
        // debug!("CUR: {} = {:?}, TEMP: {}", c, cur_token, temp);
        if let Some(token) = get_token(chars.peek(), &mut tokens) {
            tokens.push(token);
        }

        chars.next();
    }

    debug!("Tokens: {:?}", tokens);
    debug!("--------------------");

    Ok(tokens)
}
