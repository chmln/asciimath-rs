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
                temp.parse::<f64>().unwrap(),
            )));

            temp.clear();
        }

        if ch.is_alphabetic() {
            tokens.push(Token::Variable(Variable {
                name: ch.to_string(),
            }));
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

        debug!("TEMP: {}", temp);

        if !temp.is_empty() {
            if c == '(' {
                tokens.push(Token::Function(Function::new(temp.clone())));
                temp.clear();
                len -= 1;
                continue;
            }
            else {
                // TODO: maybe implement implicit multiplication
                tokens.append(&mut parse_implicit(&temp));
            }

            temp.clear();
        }

        let t_mut = &mut tokens;

        if let Some(token) = get_token(c) {
            if token == Token::Operator(Operator::Substract) {
                match t_mut.last() {
                    Some(Token::Comma)
                    | Some(Token::LeftParenthesis)
                    | Some(Token::Function(_))
                    | Some(Token::Operator(_))
                    | None => {
                        t_mut.push(Token::Number(Number::new(-1)));
                        t_mut.push(Token::Operator(Operator::Multiply));
                    },
                    _ => {
                        t_mut.push(token);
                    },
                }
            }
            else {
                t_mut.push(token);
            }
        }

        len -= 1;
    }

    debug!("Final Tokens: {:?}", tokens);
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
fn test_parse_impl() {
    assert_eq!(
        parse_implicit("1"),
        vec![Token::Number(Number::new(1.0))]
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
