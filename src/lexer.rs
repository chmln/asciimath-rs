use ast::{resolve_fn, resolve_var, NumericLiteral, Scope};
use error::Error;
use tokens::{Function, Number, Operator, Token, TokenList, Variable};

fn resolve_vars(expr: &str, scope: &Scope) -> TokenList {
    let mut tokens: TokenList = Vec::with_capacity(expr.len() * 2);
    let mut chars = expr.chars();
    let mut temp = String::with_capacity(expr.len());

    loop {
        let ch = chars.next();
        match ch {
            Some(ch) => {
                if temp.is_empty() || resolve_var(&temp, scope).is_err() {
                    temp.push(ch);
                    continue;
                }
            },
            _ => {
                if temp.is_empty() {
                    break;
                }
            },
        }

        if !temp.is_empty() {
            if resolve_var(&temp, scope).is_ok() {
                tokens.push(Token::Variable(Variable {
                    name: temp.clone(),
                }));
                tokens.push(Token::Operator(Operator::Multiply));
                if let Some(ch) = ch {
                    temp = ch.to_string();
                }
                else {
                    temp.clear();
                }
            }
            else {
                for c in temp.chars() {
                    tokens.push(Token::Variable(Variable {
                        name: c.to_string(),
                    }));
                    tokens.push(Token::Operator(Operator::Multiply));
                }
                break;
            }
        }
    }

    tokens
}

fn parse_implicit(expr: &str, scope: &Scope) -> Result<TokenList, Error> {
    let mut tokens: TokenList = Vec::with_capacity(expr.len() * 2);
    let mut digits = String::new();
    let mut word = String::new();
    let mut chars_left = expr.len();

    for ch in expr.chars() {
        if ch.is_digit(10) || ch == '.' {
            if !word.is_empty() {
                tokens.append(&mut resolve_vars(&word, scope));
                word.clear();
            }

            digits.push(ch);
            if chars_left > 1 {
                chars_left -= 1;
                continue;
            }
        }

        if !digits.is_empty() {
            tokens.push(Token::Number(Number::new(digits
                .parse::<NumericLiteral>()
                .map_err(|_e| Error::InvalidToken(digits.clone()))?)));
            tokens.push(Token::Operator(Operator::Multiply));

            digits.clear();
        }

        if ch.is_alphabetic() {
            word.push(ch);
            if chars_left > 1 {
                chars_left -= 1;
                continue;
            }
        }

        if !word.is_empty() {
            tokens.append(&mut resolve_vars(&word, scope));

            word.clear();
        }

        chars_left -= 1;
    }

    Ok(tokens)
}

fn get_token(ch: char) -> Option<Token> {
    match ch {
        '+' => Some(Token::Operator(Operator::Add)),
        '-' => Some(Token::Operator(Operator::Substract)),
        '*' => Some(Token::Operator(Operator::Multiply)),
        '/' => Some(Token::Operator(Operator::Divide)),
        '^' => Some(Token::Operator(Operator::Exponentiate)),
        '(' => Some(Token::LeftParenthesis),
        ')' => Some(Token::RightParenthesis),
        ',' => Some(Token::Comma),
        _ => None,
    }
}

pub fn tokenize<'a>(expr: &str, scope: &'a Scope) -> Result<TokenList, Error> {
    let trimmed = expr.replace(" ", "");
    let mut len = trimmed.len();
    let mut chars = trimmed.chars();

    let mut tokens = Vec::with_capacity(len);
    let mut temp = String::new();

    while let Some(c) = chars.next() {
        if c.is_alphanumeric() || c == '_' || c == '.' {
            temp.push(c);

            if len > 1 {
                len -= 1;
                continue;
            }
        }

        let cur_token = get_token(c);
        let t_mut = &mut tokens;

        debug!("CUR: {:?}, TEMP: {}", cur_token, temp);

        if !temp.is_empty() {
            if cur_token == Some(Token::LeftParenthesis)
                && resolve_fn(&temp, scope).is_ok()
            {
                t_mut.push(Token::Function(Function::new(temp.clone())));
                temp.clear();
                len -= 1;
                continue;
            }
            else {
                if t_mut.last() == Some(&Token::RightParenthesis) {
                    t_mut.push(Token::Operator(Operator::Multiply));
                }
                t_mut.append(&mut parse_implicit(&temp, scope)?);
                if cur_token != Some(Token::LeftParenthesis) {
                    t_mut.pop();
                }
            }

            temp.clear();
        }

        if let Some(token) = cur_token {
            match token {
                // Negative numbers
                Token::Operator(Operator::Substract) => match t_mut.last() {
                    Some(Token::Comma)
                    | Some(Token::LeftParenthesis)
                    | Some(Token::Function(_))
                    | Some(Token::Operator(_))
                    | None => {
                        t_mut.push(Token::Number(Number::new(-1)));
                        t_mut.push(Token::Operator(Operator::Multiply));
                    },
                    _ => {
                        // just regular subtraction
                        t_mut.push(token);
                    },
                },
                // not subtraction - proceed
                _ => {
                    t_mut.push(token);
                },
            }
        }

        len -= 1;
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
    assert_eq!(tokens, expected_tokens)
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
        resolve_vars("abcd", &vars),
        vec![
            Token::Variable(Variable {
                name: "abcd".to_string(),
            }),
            Token::Operator(Operator::Multiply),
        ]
    );

    assert_eq!(
        resolve_vars("abcd", &vars_2),
        vec![
            Token::Variable(Variable {
                name: "ab".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "cd".to_string(),
            }),
            Token::Operator(Operator::Multiply),
        ]
    );

    assert_eq!(
        resolve_vars("abcd", &vars_3),
        vec![
            Token::Variable(Variable {
                name: "abc".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "d".to_string(),
            }),
            Token::Operator(Operator::Multiply),
        ]
    );

    assert_eq!(
        resolve_vars("abcd", &vars_4),
        vec![
            Token::Variable(Variable {
                name: "a".to_string(),
            }),
            Token::Operator(Operator::Multiply),
            Token::Variable(Variable {
                name: "bcd".to_string(),
            }),
            Token::Operator(Operator::Multiply),
        ]
    );

    assert_eq!(
        resolve_vars("abcd", &scope!{}),
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
            Token::Operator(Operator::Multiply),
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
