use std::{fmt};

pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Exponentiate,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Substract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Exponentiate => "^",
            }
        )
    }
}

pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new (value: f64) -> Number {
        Number {
            value
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Number(Number),
    LeftParenthesis,
    RightParenthesis,
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let trimmed = expr.replace(" ", "");
    let mut chars = trimmed.chars();

    let mut temp = String::new();

    loop {
        if let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                temp.push(c);
                continue;
            }

            if temp.len() > 0 {
                if let Ok(num) = temp.parse::<f64>() {
                    tokens.push(Token::Number(Number { value: num }))
                }

                temp.clear()
            }

            match c {
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => tokens.push(Token::Operator(Operator::Substract)),
                '*' => tokens.push(Token::Operator(Operator::Multiply)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),

                // todo parens
                '(' => tokens.push(Token::LeftParenthesis),
                ')' => tokens.push(Token::RightParenthesis),
                _ => {}
            }
        } else {
            // todo: refactor
            if temp.len() > 0 {
                if let Ok(num) = temp.parse::<f64>() {
                    tokens.push(Token::Number(Number { value: num }))
                }
            }

            break;
        }
    }

    tokens
}


