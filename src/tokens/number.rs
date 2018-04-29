#[derive(Debug)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new<T>(value: T) -> Number
    where
        T: Into<f64>,
    {
        Number {
            value: value.into(),
        }
    }
}
