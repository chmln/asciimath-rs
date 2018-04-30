use tokens::Token;

pub struct Node {
    pub token: Token,
    pub args: Option<Vec<Node>>,
}
