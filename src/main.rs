use math_lib::{derivative::lexer::tokenize, derivative::parser::parser::parse_fully};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = tokenize("arcsin(e^(cos(ln(x^(3^x)))))".to_string(), "x".to_string());
    let res = parse_fully(&tokens)?;
    println!("{:?}", res);
    Ok(())
}
