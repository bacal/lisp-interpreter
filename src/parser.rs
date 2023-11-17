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
    MissingFunctionBody,
    MissingFunctionName,
    MissingArguments,
    UnexpectedEndWhileParsing,
}

enum ParseResult{
    InsertedFunc(String),
    EvalUnary(f64),
}

enum UnaryOp{
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
struct Function{
    args: Vec<Token>,
    expr: Vec<Token>,
}
pub struct Parser {
    variables: HashMap<String,f64>,
    functions: HashMap<String, Function>,
    fn_parsed: bool,
}

impl Parser {
    pub fn new() -> Self{
        Self{
            variables: HashMap::new(),
            functions: HashMap::new(),
            fn_parsed: false,
        }
    }
    pub fn evaluate(&mut self, tokens: &[Token]) {
        let mut iter = tokens.iter().peekable();
        if let Some(t) = iter.peek(){
            if **t!=Token::LeftParen{
                println!("ParseError: {:?}", LispParseError::MissingLeftParen)
            }
        }

        let res = self.evaluate_parenthesis(&mut iter);
        println!("{}",res.unwrap_or(0.0));
        // match res{
        //     Ok(val) => {
        //         match val{
        //             ParseResult::EvalUnary(numeric_result) => println!("{numeric_result}"),
        //             ParseResult::InsertedFunc(function_name) => println!("{function_name}"),
        //         }
        //     }
        //     Err(e) => println!("ParseError: {:?}", e),
        // }
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
                Token::Defvar => res += self.insert_var(iter)?,
                Token::Defun => self.push_function(iter)?,
                Token::Symbol(s) => res += self.eval_function(s,iter)?,
                _ => {},
            }
        }
        match iter.peek(){
            Some(Token::RightParen) => {
                iter.next();
                Ok(res)
            },
            Some(_) =>Err(LispParseError::MissingRightParen),
            None => {
                if self.fn_parsed{
                    self.fn_parsed = false;
                    Ok(res)
                }
                else{
                    Err(LispParseError::MissingRightParen)
                }
            }
        }

    }

    fn insert_var(&mut self, iter: &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError>{
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
        let mut res : Option<f64> = None;
        while let Ok(arg) = self.get_value(iter){
            if res.is_none(){
                let _ = res.insert(arg);
            }
            else{
                let t = res.get_or_insert_with(|| 0.0);
                match operation {
                    UnaryOp::Addition => *t+=arg,
                    UnaryOp::Subtraction => *t-=arg,
                    UnaryOp::Multiplication => *t*=arg,
                    UnaryOp::Division => *t/=arg,
                }
            }
        }
        match res{
            Some(res) => Ok(res),
            None => Err(LispParseError::InvalidArgument),
        }
    }

    fn get_value(&mut self, iter:  &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError>{
        match iter.next_if(|t| **t!= Token::RightParen) {
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
            Some(_) | None => Err(LispParseError::InvalidArgument),
        }
    }
    fn push_function(&mut self, iter: &mut Peekable<Iter<Token>>) -> Result<(),LispParseError> {
        if let Some(Token::Symbol(fn_name)) = iter.next(){
            let mut args = vec![];
            while let Some(t) = iter.next_if(|t| **t != Token::LeftParen){
                args.push(t.clone());
            }
            let mut expr: Vec<Token> = vec![];
            let mut left_paren = 1;
            while let Some(t) = iter.next_if(|t| **t != Token::RightParen){
                if *t == Token::LeftParen{
                    left_paren+=1;
                }
                expr.push(t.clone());
            }
            let mut right_paren = 0;
            while let Some(_) = iter.next_if(|t| **t == Token::RightParen){
                expr.push(Token::RightParen);
                right_paren+=1
            }
            if right_paren != left_paren {
                return Err(LispParseError::MissingRightParen);
            }
            self.functions.insert(fn_name.to_string(),Function{args,expr});
            self.fn_parsed = true;
            Ok(())
        }
        else{
            Err(LispParseError::MissingFunctionName)
        }
    }
    fn eval_function(&mut self, function_name: &String, iter: &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError> {
        let mut args = vec![];
        while let Some(t) = iter.next_if(|t| **t != Token::RightParen){
            args.push(t.clone());
        }
        match self.functions.get(function_name) {
            Some(fun) => {
                let mut expr = fun.expr.clone();
                for (arg, replace_arg) in fun.args.iter().zip(args) {
                    expr.iter_mut().for_each(|t| {
                        if t == arg{
                            *t = (replace_arg).clone()
                        }
                    });
                }
                self.evaluate_parenthesis(&mut expr.iter().peekable())
            },
            None => Err(LispParseError::InvalidFunction(function_name.clone()))
        }
    }
}