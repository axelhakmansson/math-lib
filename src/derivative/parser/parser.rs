use thiserror::Error;
use super::{NodeToken, Node, Token};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid expression")]
    InvalidExpression,
}



fn full_parse(tokens: &Vec<Token>) -> Result<NodeToken, ParseError> {
    let paren_free = parse_paren(tokens)?;
    let function = parse_func(&paren_free)?;
    let rest = parse_rest(&function)?;
    todo!()    
}


fn parse_paren(tokens: &[Token]) -> Result<Vec<NodeToken>, ParseError> {
    let mut sub_part: Vec<Token> = vec![];
    let mut res: Vec<NodeToken> = vec![];
    let mut paren_count = 0;

    for token in tokens {
        match token {
            Token::LParen => {
                paren_count += 1;
            }
            Token::RParen => {
                paren_count -= 1;

                if paren_count < 0 {
                    return Err(ParseError::InvalidExpression);
                }

                if paren_count == 0 {
                    let sub_res = full_parse(&sub_part)?;
                    res.push(sub_res);
                    sub_part.clear();
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
        return Err(ParseError::InvalidExpression);
    }

    Ok(res)
}

fn parse_func(tokens: &[NodeToken]) -> Result<Vec<NodeToken>, ParseError> {
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
    todo!()
}

fn parse_rest(tokens: &[NodeToken]) -> Result<Node, ParseError> {
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

    for i in (1..tokens.len()).rev() {
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

    for i in (1..tokens.len()).rev() {
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

    if let NodeToken::Token(Token::Sub) = tokens[0] {
        let right = parse_rest(&tokens[1..])?;
        return Ok(Node::Neg(Box::new(right)));
    }

    Err(ParseError::InvalidExpression)
}


//tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivative::lexer::tokenize;

    #[test]
    fn test_parse_rest_1() {
        let tokens = tokenize("2 + 3 * 4".to_string(), "x".to_string());
        let node_tokens: Vec<NodeToken> = tokens.iter().map(|token| NodeToken::Token(token.clone())).collect();
        let res = parse_rest(&node_tokens).unwrap();
        
        assert_eq!(res, Node::Add(
            Box::new(Node::Number(2.0)),
            Box::new(Node::Mul(
                Box::new(Node::Number(3.0)),
                Box::new(Node::Number(4.0))
            ))
        ));
    }

    #[test]
    fn test_parse_rest_2() {
        let tokens = tokenize("-3^2".to_string(), "x".to_string());
        let node_tokens: Vec<NodeToken> = tokens.iter().map(|token| NodeToken::Token(token.clone())).collect();
        let res = parse_rest(&node_tokens).unwrap();
        
        assert_eq!(res, Node::Pow(
            Box::new(Node::Neg(
                Box::new(Node::Number(3.0))
            )),
            Box::new(Node::Number(2.0))
        ));
    }
}