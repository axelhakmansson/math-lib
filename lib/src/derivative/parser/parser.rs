use std::vec;

use thiserror::Error;
use super::{NodeToken, Node, Token};

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Invalid expression")]
    InvalidExpression,
    #[error("Uneven parantheses")]
    UnevenParantheses,
    #[error("No node after function")]
    NoNodeAfterFunc,
}



pub fn parse_fully(tokens: &Vec<Token>) -> Result<Node, ParseError> {
    let paren_free = parse_paren(tokens)?;
    let function = parse_func(&paren_free)?;
    let rest = parse_rest(&function)?;
    Ok(rest)
}


pub fn parse_paren(tokens: &[Token]) -> Result<Vec<NodeToken>, ParseError> {
    let mut sub_part: Vec<Token> = vec![];
    let mut res: Vec<NodeToken> = vec![];
    let mut paren_count = 0;

    for token in tokens {
        match token {
            Token::LParen => {
                if paren_count > 0 {
                    sub_part.push(token.clone());
                }
                paren_count += 1;
            }
            Token::RParen => {
                paren_count -= 1;

                if paren_count < 0 {
                    return Err(ParseError::UnevenParantheses);
                }

                if paren_count == 0 {
                    let sub_res = parse_fully(&sub_part)?;
                    res.push(NodeToken::Node(sub_res));
                    sub_part.clear();
                } else {
                    sub_part.push(token.clone());
                }
            }
            _ => {
                if paren_count == 0 {
                    res.push(NodeToken::Token(token.clone()));
                } else {
                    sub_part.push(token.clone());
                }
            }
        }
    }

    if paren_count != 0 {
        return Err(ParseError::UnevenParantheses);
    }

    Ok(res)
}

pub fn parse_func(tokens: &[NodeToken]) -> Result<Vec<NodeToken>, ParseError> {
    /*
        Ln
        Log
        Sin
        Cos
        Tan
        Arcsin
        Arccos
        Arctan
        Sqrt
     */
    let mut res = vec![];
    let mut skip = false;

    for i in 0..tokens.len() {
        if let NodeToken::Token(token) = &tokens[i] {
            match token {
                Token::Ln => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Ln(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Log => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Log(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Sin => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Sin(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Cos => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Cos(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Tan => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Tan(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Arcsin => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Arcsin(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Arccos => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Arccos(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Arctan => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Arctan(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                Token::Sqrt => {
                    if let NodeToken::Node(node) = &tokens[i+1] {
                        res.push(NodeToken::Node(Node::Sqrt(Box::new(node.clone()))));
                        skip = true;
                    } else {
                        return Err(ParseError::NoNodeAfterFunc);
                    }
                }
                _ => {
                    if skip {
                        skip = false;
                        continue;
                    }
                    res.push(NodeToken::Token(token.clone()));                
                }
            }
        } else {
            if skip {
                skip = false;
                continue;
            }
            res.push(tokens[i].clone());
        }
    }
    Ok(res)
}

pub fn parse_rest(tokens: &[NodeToken]) -> Result<Node, ParseError> {

    if tokens.len() == 1 {
        match &tokens[0] {
            NodeToken::Token(token) => {
                match token {
                    Token::Number(num) => {
                        return Ok(Node::Number(num.clone()));
                    }
                    Token::Variable(var) => {
                        return Ok(Node::Variable(var.clone()));
                    }
                    Token::Constant(constant) => {
                        return Ok(Node::Constant(constant.clone()));
                    }
                    Token::EConstant => {
                        return Ok(Node::EConstant);
                    }
                    _ => {
                        return Err(ParseError::InvalidExpression);
                    }
                }
            }
            NodeToken::Node(node) => {
                return Ok(node.clone());
            }
        }
    }

    for i in 1..tokens.len() {
        if let NodeToken::Token(token) = &tokens[i] {
            match token {
                Token::Add => {
                    let left = parse_rest(&tokens[..i])?;
                    let right = parse_rest(&tokens[i+1..])?;
                    return Ok(Node::Add(Box::new(left), Box::new(right)));
                }
                Token::Sub => {

                    if let Some(token) = tokens[i-1].token() {
                        if token == Token::Add || token == Token::Sub || token == Token::Mul || token == Token::Div || token == Token::Pow {
                            continue;
                        }
                    }

                    let left = parse_rest(&tokens[..i])?;
                    let right = parse_rest(&tokens[i+1..])?;
                    return Ok(Node::Sub(Box::new(left), Box::new(right)));
                }
                _ => {}
            }
        }
    }

    for i in 1..tokens.len() {
        if let NodeToken::Token(token) = &tokens[i] {
            match token {
                Token::Mul => {
                    let left = parse_rest(&tokens[..i])?;
                    let right = parse_rest(&tokens[i+1..])?;
                    return Ok(Node::Mul(Box::new(left), Box::new(right)));
                }
                Token::Div => {
                    let left = parse_rest(&tokens[..i])?;
                    let right = parse_rest(&tokens[i+1..])?;
                    return Ok(Node::Div(Box::new(left), Box::new(right)));
                }
                _ => { }
            }
        }
    }

    if let NodeToken::Token(Token::Sub) = tokens[0] {
        let right = parse_rest(&tokens[1..])?;
        return Ok(Node::Neg(Box::new(right)));
    }

    for i in 1..tokens.len() {
        if let NodeToken::Token(token) = &tokens[i] {
            match token {
                Token::Pow => {
                    let left = parse_rest(&tokens[..i])?;
                    let right = parse_rest(&tokens[i+1..])?;
                    return Ok(Node::Pow(Box::new(left), Box::new(right)));
                }
                _ => { }
            }
        }
    }
    Err(ParseError::InvalidExpression)
}

