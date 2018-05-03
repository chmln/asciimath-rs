use ast::NumericLiteral;

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: NumericLiteral,
}

impl Number {
    pub fn new<T>(value: T) -> Number
    where
        T: Into<NumericLiteral>,
    {
        Number {
            value: value.into(),
        }
    }
}
