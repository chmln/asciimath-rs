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

#[test]
fn lexer_negative_numbers() {
    let tokens = tokenize("x+-1", &Scope::new()).unwrap();
    let expected_tokens = vec![
        Token::Variable(Variable {
            name: "x".to_string(),
        }),
        Token::Operator(Operator::Add),
        Token::Number(Number::new(-1.0)),
        Token::Operator(Operator::Multiply),
        Token::Number(Number::new(1.0)),
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_resolve_vars() {
    let vars = scope!{
        "abcd" => 1
    };

    let vars_2 = scope!{
        "ab" => 2,
        "cd" => 3
    };

    let vars_3 = scope!{
        "abc" => 2,
        "d" => 3
    };

    let vars_4 = scope!{
        "a" => 2,
        "bcd" => 3
    };

    assert_eq!(
        tokenize("abcd", &vars).unwrap(),
        vec![Token::Variable(Variable {
            name: "abcd".to_string(),
        })]
    );

    assert_eq!(
        tokenize("abcd", &vars_2).unwrap(),
        vec![
            Token::Variable(Variable {
                name: "ab".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "cd".to_string(),
            }),
        ]
    );

    assert_eq!(
        tokenize("abcd", &vars_3).unwrap(),
        vec![
            Token::Variable(Variable {
                name: "abc".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "d".to_string(),
            }),
        ]
    );

    assert_eq!(
        tokenize("abcd", &vars_4).unwrap(),
        vec![
            Token::Variable(Variable {
                name: "a".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "bcd".to_string(),
            }),
        ]
    );

    assert_eq!(
        tokenize("abcd", &scope!{}).unwrap(),
        vec![
            Token::Variable(Variable {
                name: "a".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "b".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "c".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "d".to_string(),
            }),
        ]
    );
}

#[test]
fn lexer_word_variables() {
    assert_eq!(
        tokenize("quantity*2", &scope!{ "quantity" => 1 }),
        Ok(vec![
            Token::Variable(Variable {
                name: "quantity".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Number(Number::new(2.0)),
        ])
    );

    assert_eq!(
        tokenize("2quantity", &scope!{ "quantity" => 1 }),
        Ok(vec![
            Token::Number(Number::new(2.0)),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "quantity".to_string(),
            }),
        ])
    );
}

#[test]
fn test_implicit_multiplication() {
    let mut scope = scope!{ "x" => 0 };

    assert_eq!(
        tokenize("1", &Scope::new()).unwrap(),
        vec![Token::Number(Number::new(1.0))]
    );
    assert_eq!(
        tokenize("3x^2", &Scope::new()).unwrap(),
        vec![
            Token::Number(Number::new(3)),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "x".to_string(),
            }),
            Token::Operator(Operator::Exponentiate),
            Token::Number(Number::new(2)),
        ]
    );
    assert_eq!(
        tokenize("4(x+3)2", &Scope::new()).unwrap(),
        vec![
            Token::Number(Number::new(4)),
            Token::Operator(Operator::Multiply),
            Token::LeftParenthesis,
            Token::Variable(Variable {
                name: "x".to_string(),
            }),
            Token::Operator(Operator::Add),
            Token::Number(Number::new(3)),
            Token::RightParenthesis,
            Token::Operator(Operator::Multiply),
            Token::Number(Number::new(2)),
        ]
    );

    assert_eq!(
        tokenize("2x(x+3)", &scope).unwrap(),
        vec![
            Token::Number(Number::new(2)),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "x".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::LeftParenthesis,
            Token::Variable(Variable {
                name: "x".to_string(),
            }),
            Token::Operator(Operator::Add),
            Token::Number(Number::new(3)),
            Token::RightParenthesis,
        ]
    );
    assert_eq!(
        tokenize("x^(2y+3z)", &Scope::new()).unwrap(),
        vec![
            Token::Variable(Variable {
                name: "x".to_string(),
            }),
            Token::Operator(Operator::Exponentiate),
            Token::LeftParenthesis,
            Token::Number(Number::new(2)),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "y".to_string(),
            }),
            Token::Operator(Operator::Add),
            Token::Number(Number::new(3)),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "z".to_string(),
            }),
            Token::RightParenthesis,
        ]
    )
}

#[test]
fn lexer_floats() {
    let tokens = tokenize("max(1,3,25.75,10.5)", &Scope::new()).unwrap();
    let expected_tokens = vec![
        Token::Function(Function::new("max".to_string())),
        Token::Number(Number::new(1.0)),
        Token::Comma,
        Token::Number(Number::new(3.0)),
        Token::Comma,
        Token::Number(Number::new(25.75)),
        Token::Comma,
        Token::Number(Number::new(10.5)),
        Token::RightParenthesis,
    ];
    assert_eq!(tokens, expected_tokens)
}
