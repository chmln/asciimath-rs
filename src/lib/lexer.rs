use std::{cmp, fmt};

#[derive(Clone, Copy, Eq)]
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

impl cmp::PartialEq for Operator {
    fn eq(&self, other: &Operator) -> bool {
        self.get_precedence() == other.get_precedence()
    }
}
impl cmp::PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<cmp::Ordering> {
        Some(
            self.get_precedence()
                .cmp(&other.get_precedence()),
        )
    }
}

impl Operator {
    pub fn get_precedence(&self) -> i8 {
        match self {
            Operator::Add => 2,
            Operator::Substract => 2,
            Operator::Multiply => 3,
            Operator::Divide => 3,
            Operator::Exponentiate => 4,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        *self == Operator::Exponentiate
    }
}
#[derive(Debug)]
pub struct Number {
    pub value: f64,
}

pub fn num<T>(value: T) -> Number
where
    T: Into<f64>,
{
    Number {
        value: value.into(),
    }
}

//
#[derive(Debug)]
pub struct Variable {
    pub name: String,
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Number(Number),
    Variable(Variable),
    LeftParenthesis,
    RightParenthesis,
}

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
            tokens.push(Token::Number(num(temp.parse::<f64>().unwrap())));
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
            if c == '(' {
                tokens.push(Token::Operator(Operator::Multiply));
            }
        }

        match c {
            '+' => tokens.push(Token::Operator(Operator::Add)),
            '-' => tokens.push(Token::Operator(Operator::Substract)),
            '*' => tokens.push(Token::Operator(Operator::Multiply)),
            '/' => tokens.push(Token::Operator(Operator::Divide)),
            '^' => tokens.push(Token::Operator(Operator::Exponentiate)),
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            _ => {}
        }

        len -= 1;
    }

    println!("{:?}", tokens);

    tokens
}
