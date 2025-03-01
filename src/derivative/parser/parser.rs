use thiserror::Error;
use crate::derivative::lexer::Token;

use super::node::NodeToken;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid expression")]
    InvalidExpression,
}



fn full_parse(tokens: &Vec<Token>) -> Result<NodeToken, ParseError> {
    let paren_free = parse_paren(tokens)?;
    let function = parse_func(paren_free)?;
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

fn parse_func(tokens: Vec<NodeToken>) -> Result<NodeToken, ParseError> {
    todo!()
}
