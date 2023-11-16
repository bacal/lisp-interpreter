use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::scanner::Token;

#[derive(Debug)]
pub enum LispParseError{
    MissingOperand,
    InvalidSymbol(String),
    InvalidArgument,
    MissingSymbol,
    InvalidFunction(String),
    MissingLeftParen,
    MissingRightParen,
}

enum UnaryOp{
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub struct Parser {
    variables: HashMap<String,f64>,
}

impl Parser {
    pub fn new() -> Self{
        Self{
            variables: HashMap::new()
        }
    }
    pub fn evaluate(&mut self, tokens: &[Token]) -> Result<f64,LispParseError> {
        let mut iter = tokens.iter().peekable();
        if let Some(t) = iter.peek(){
            if **t!=Token::LeftParen{
                return Err(LispParseError::MissingLeftParen)
            }
        }
        self.evaluate_parenthesis(&mut iter)
    }

    fn evaluate_parenthesis(&mut self, iter: &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError> {
        let mut res = 0.0;
        if iter.size_hint().0 == 0{
            return Err(LispParseError::MissingOperand);
        }
        while let Some(t) = iter.next_if(|t| **t != Token::RightParen) {
            match t {
                Token::Plus => res += self.eval_unary(iter, UnaryOp::Addition)?,
                Token::Minus => res += self.eval_unary(iter, UnaryOp::Subtraction)?,
                Token::Asterisk => res += self.eval_unary(iter, UnaryOp::Multiplication)?,
                Token::ForwardSlash => res += self.eval_unary(iter, UnaryOp::Division)?,
                Token::Defvar => res += self.insert_data(iter)?,
                Token::Symbol(s) => return Err(LispParseError::InvalidFunction(s.clone())),
                _ => {},
            }
        }
        if let Some(t) = iter.peek(){
            if **t != Token::RightParen{
                return Err(LispParseError::MissingRightParen);
            }
        }
        else if iter.peek().is_none(){
            return Err(LispParseError::MissingRightParen);
        }
        iter.next();
        Ok(res)
    }

    fn insert_data(&mut self, iter: &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError>{
        match iter.next(){
            Some(Token::Symbol(var_name)) => {
                let val = match iter.next() {
                    Some(Token::LeftParen) => self.evaluate_parenthesis(iter).expect("FatalError: failed to parse"),
                    Some(Token::Number(n)) => *n,
                    Some(_) | None => return Err(LispParseError::InvalidArgument),
                };
                self.variables.insert(var_name.clone(), val);
                Ok(val)
            }
            Some(Token::Number(_)) => Err(LispParseError::InvalidArgument),
            _ =>  Err(LispParseError::MissingSymbol)
        }
    }

    fn eval_unary(&mut self, iter: &mut Peekable<Iter<Token>>, operation: UnaryOp) -> Result<f64, LispParseError> {
        let (a, b) = (self.get_value(iter)?,self.get_value(iter)?);
        Ok(match operation {
            UnaryOp::Addition => a + b,
            UnaryOp::Subtraction => a - b,
            UnaryOp::Multiplication => a * b,
            UnaryOp::Division => a / b,
        })
    }

    fn get_value(&mut self, iter:  &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError>{
        match iter.next() {
            Some(Token::Number(n)) => {
                Ok(*n)
            }
            Some(Token::Symbol(s)) => {
                if let Some(val) = self.variables.get(s.as_str()){
                    Ok(*val)
                }
                else{
                     Err(LispParseError::InvalidSymbol(s.clone()))
                }
            }
            Some(Token::LeftParen) => {
                Ok(self.evaluate_parenthesis(iter)?)
            }
            Some(_) | None => Err(LispParseError::InvalidArgument)
        }
    }
}
