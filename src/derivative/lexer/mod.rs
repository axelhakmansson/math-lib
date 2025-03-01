#[derive(Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,    
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Ln,
    Log,                    // senare, log_a definerar bas annars 10 https://en.wikipedia.org/wiki/Derivative
    Pow,
    Constant(String),
    Variable(String),
    EConstant,
}


pub fn tokenize(input: String, variable: String) -> Vec<Token> { // t.ex. sin(x^2.0)*e^(2.0*x)    viktigt med alla tecken och . vid nummer för parse
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' | '.' => {
                            num.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' => {
                            name.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if name == variable {
                    tokens.push(Token::Variable(name));
                } else {
                    match name.as_str() {
                        "sin" => tokens.push(Token::Sin),
                        "cos" => tokens.push(Token::Cos),
                        "tan" => tokens.push(Token::Tan),
                        "arcsin" => tokens.push(Token::Arcsin),
                        "arccos" => tokens.push(Token::Arccos),
                        "arctan" => tokens.push(Token::Arctan),
                        "ln" => tokens.push(Token::Ln),
                        "log" => tokens.push(Token::Log),
                        "e" => tokens.push(Token::EConstant),
                        _ => tokens.push(Token::Constant(name)),
                    }
                }
            }
            '+' => {
                tokens.push(Token::Add);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Sub);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Mul);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Div);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Pow);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }
    tokens
}