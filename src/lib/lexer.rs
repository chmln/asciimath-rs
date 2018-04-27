use std::{cmp, fmt};

#[derive(Clone, Copy, Eq)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Exponentiate,
    OpeningParenthesis,
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
                Operator::OpeningParenthesis => "(",
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
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Operator {
    fn cmp(&self, other: &Operator) -> cmp::Ordering {
        let prec = self.get_precedence();
        let other_prec = other.get_precedence();
        if prec > other_prec {
            cmp::Ordering::Greater
        } else if prec == other_prec {
            cmp::Ordering::Equal
        } else {
            cmp::Ordering::Less
        }
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
            _ => -1,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        match self {
            Operator::Exponentiate => true,
            _ => false,
        }
    }
}

pub struct Number {
    pub value: f64,
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
                '^' => tokens.push(Token::Operator(Operator::Exponentiate)),
                '(' => {
                    tokens.push(Token::Operator(Operator::OpeningParenthesis))
                }
                ')' => tokens.push(Token::RightParenthesis),
                _ => {}
            }
        } else {
            // todo: refactor
            if temp.len() > 0 {
                if let Ok(value) = temp.parse::<f64>() {
                    tokens.push(Token::Number(Number { value }))
                }
            }

            break;
        }
    }

    tokens
}
