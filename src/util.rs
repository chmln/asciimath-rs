use std::{iter::Peekable, str};

pub fn consume_while<F>(it: &mut Peekable<str::Chars>, x: F) -> String
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
