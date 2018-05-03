use ast::NumericLiteral;
use tokens::{Function, Number, Operator, Token, TokenList, Variable};

fn parse_implicit(expr: &str) -> TokenList {
    let mut tokens: TokenList = Vec::with_capacity(expr.len());
    let mut temp = String::new();
    let mut chars_left = expr.len();

    for ch in expr.chars() {
        if ch.is_digit(10) || ch == '.' {
            temp.push(ch);
            if chars_left > 1 {
                chars_left -= 1;
                continue;
            }
        }

        if !temp.is_empty() {
            tokens.push(Token::Number(Number::new(
                temp.parse::<NumericLiteral>().unwrap(),
            )));
            tokens.push(Token::Operator(Operator::Multiply));

            temp.clear();
        }

        if ch.is_alphabetic() {
            tokens.push(Token::Variable(Variable {
                name: ch.to_string(),
            }));
            tokens.push(Token::Operator(Operator::Multiply));
        }

        chars_left -= 1;
    }

    tokens
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

pub fn tokenize(expr: &str) -> TokenList {
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
                && temp.parse::<NumericLiteral>().is_err()
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
                t_mut.append(&mut parse_implicit(&temp));
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

    tokens
}

#[test]
fn lexer_negative_numbers() {
    let tokens = tokenize("x+-1");
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
fn test_implicit_multiplication() {
    assert_eq!(
        tokenize("1"),
        vec![Token::Number(Number::new(1.0))]
    );
    assert_eq!(
        tokenize("3x^2"),
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
        tokenize("4(x+3)2"),
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
        tokenize("x^(2y+3z)"),
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
    let tokens = tokenize("max(1,3,25.75,10.5)");
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
