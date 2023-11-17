#[derive(Debug,PartialEq,Clone)]
pub enum Token{
    LeftParen,
    RightParen,
    Symbol(String),
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    String(String),
    Number(f64),
    Defun,
    Defvar,
    Exp,
}
pub fn scan_tokens(input: &str) -> Vec<Token>{
    let mut chars = input.chars().enumerate().peekable();
    let mut tokens = vec![];
    let mut buffer = String::new();
    while let Some((_i,c)) = chars.next(){
        match c {
            ' ' | '\t' |'\n' => {}
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::ForwardSlash),
            '\'' => {
                while let Some((_, c)) = chars.next_if(|(_, c)| *c != '\'') {
                    buffer.push(c);
                }
                tokens.push(Token::String(buffer.clone()));
                buffer.clear();
                chars.next();
            }
            _ => {
                buffer.push(c);
                while let Some((_,c)) = chars.next_if(|(_,c)| c.is_alphanumeric()){
                    buffer.push(c);
                }
                if let Ok(num) = buffer.parse::<f64>(){
                    tokens.push(Token::Number(num));
                }
                else{
                    match buffer.as_str(){
                        "defun" => tokens.push(Token::Defun),
                        "defvar" => tokens.push(Token::Defvar),
                        _ => tokens.push(Token::Symbol(buffer.clone())),

                    }
                }
                buffer.clear();
            },
        }
    }
    tokens
}
