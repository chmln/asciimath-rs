use tokens::{Number, Operator, Token, Variable};

pub fn parse_implicit(expr: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(expr.len());
    let mut temp = String::new();
    let mut chars_left = expr.len();

    for ch in expr.chars() {
        if ch.is_digit(10) {
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

    tokens.pop();
    tokens
}

pub fn get_token(ch: char) -> Option<Token> {
    match ch {
        '+' => Some(Token::Operator(Operator::Add)),
        '-' => Some(Token::Operator(Operator::Substract)),
        '*' => Some(Token::Operator(Operator::Multiply)),
        '/' => Some(Token::Operator(Operator::Divide)),
        '^' => Some(Token::Operator(Operator::Exponentiate)),
        '(' => Some(Token::LeftParenthesis),
        ')' => Some(Token::RightParenthesis),
        _ => None,
    }
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let trimmed = expr.replace(" ", "");
    let mut len = trimmed.len();
    let mut chars = trimmed.chars();

    let mut tokens: Vec<Token> = Vec::with_capacity(len);
    let mut temp = String::new();

    while let Some(c) = chars.next() {
        if c.is_alphanumeric() {
            temp.push(c);
            if len > 1 {
                len -= 1;
                continue;
            }
        }

        if temp.len() > 0 {
            tokens.append(&mut parse_implicit(&temp));
            temp.clear();
        }

        if let Some(recognized_token) = get_token(c) {
            tokens.push(recognized_token);
        }

        len -= 1;
    }

    println!("{:?}", tokens);

    tokens
}
