use super::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Number(f64),
    Variable(String),
    Constant(String),
    EConstant,

    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Neg(Box<Node>),

    Ln(Box<Node>),
    Log(Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Tan(Box<Node>),
    Arcsin(Box<Node>),
    Arccos(Box<Node>),
    Arctan(Box<Node>),
    Sqrt(Box<Node>),   
}

#[derive(Clone, Debug)]
pub enum NodeToken {
    Token(Token),
    Node(Node),
}

impl NodeToken {
    pub fn node(&self) -> Option<Node> {
        match self {
            NodeToken::Node(node) => Some(node.clone()),
            _ => None,
        }
    }

    pub fn token(&self) -> Option<Token> {
        match self {
            NodeToken::Token(token) => Some(token.clone()),
            _ => None,
        }
    }
}