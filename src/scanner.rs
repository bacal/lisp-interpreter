use std::fmt::Formatter;

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
    Carat,
    Dollar,
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
            '^' => tokens.push(Token::Carat),
            '$' => tokens.push(Token::Dollar),
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


impl std::fmt::Display for Token{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", match self{
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Symbol(s) => s.clone(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::ForwardSlash => "/".to_string(),
            Token::String(s) => format!("\"{}\"",s.as_str()),
            Token::Number(n) => format!("{n}"),
            Token::Defun => "defun".to_string(),
            Token::Defvar => "defvar".to_string(),
            Token::Carat => "^".to_string(),
            Token::Dollar => "$".to_string(),
        })
    }
}