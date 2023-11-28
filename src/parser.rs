use std::collections::HashMap;
use std::fmt::Formatter;
use std::iter::Peekable;
use std::slice::Iter;
use crate::scanner::Token;
use itertools::Itertools;

#[derive(Debug,Clone)]
pub enum LispParseError{
    MissingOperand,
    InvalidSymbol(String),
    InvalidArgument,
    MissingSymbol,
    InvalidFunction(String),
    MissingRightParen,
    MissingFunctionName,
    SyntaxError,
}

enum ParseResult{
    InsertedVar(String),
    InsertedFunc(String),
    EvalUnary(f64),
    ParseError(LispParseError),
}

enum UnaryOp{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponential,
}

#[derive(Debug)]
struct Function{
    args: Vec<Token>,
    expr: Vec<Token>,
}

impl std::fmt::Display for Function{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let args = self.args.iter().join(" ");
        let exprs = self.expr.iter().join(" ");

        write!(f, "Args: {}\nBody:{}",args,exprs)
    }
}


pub struct Parser {
    variables: HashMap<String,f64>,
    functions: HashMap<String, Function>,
    parse_result: ParseResult,
    fn_parsed: bool,
}

impl Parser {
    pub fn new() -> Self{
        Self{
            variables: HashMap::new(),
            functions: HashMap::new(),
            parse_result: ParseResult::InsertedFunc("nil".to_string()),
            fn_parsed: false,
        }
    }
    pub fn evaluate(&mut self, tokens: &[Token]) {
        let mut iter = tokens.iter().peekable();
        match iter.next() {
            Some(Token::Dollar) => {
                match &self.parse_result{
                    ParseResult::InsertedFunc(s) => println!("{s}",),
                    ParseResult::InsertedVar(s) => println!("'{s}",),
                    ParseResult::EvalUnary(val) => println!("{val}"),
                    ParseResult::ParseError(error) => println!("parse error: {:?}",error),
                }
            },
            Some(Token::LeftParen) => {
                let res = self.eval_parenthesis(&mut iter);
                match res {
                    Ok(_) => {
                        match &self.parse_result{
                            ParseResult::InsertedFunc(s) => println!("{s}",),
                            ParseResult::InsertedVar(s) => println!("'{s}",),
                            ParseResult::EvalUnary(val) => println!("{val}"),
                            ParseResult::ParseError(error) => println!("parse error: {:?}",error),
                        }
                    }
                    Err(e) => {
                        println!("parse error: {:?}", e);
                        self.parse_result = ParseResult::ParseError(e);
                    },
                }
            },
            Some(Token::Symbol(s)) =>{
              match s.as_str(){
                  "ftab" => self.print_function_table(),
                  _ =>{
                      println!("parse error: {:?}", LispParseError::SyntaxError);
                      self.parse_result = ParseResult::ParseError(LispParseError::SyntaxError)
                  }
              }
            },
            _ => {
                println!("parse error: {:?}", LispParseError::SyntaxError);
                self.parse_result = ParseResult::ParseError(LispParseError::SyntaxError);
            }
        }
    }

    fn eval_parenthesis(&mut self, iter: &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError> {
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
                Token::Carat => res += self.eval_unary(iter, UnaryOp::Exponential)?,
                Token::Defvar => res += self.push_var(iter)?,
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

    fn push_var(&mut self, iter: &mut Peekable<Iter<Token>>) -> Result<f64,LispParseError>{
        match iter.next(){
            Some(Token::Symbol(var_name)) => {
                let val = match iter.next() {
                    Some(Token::LeftParen) => self.eval_parenthesis(iter).expect("FatalError: failed to parse"),
                    Some(Token::Number(n)) => *n,
                    Some(_) | None => return Err(LispParseError::InvalidArgument),
                };
                self.parse_result = ParseResult::InsertedVar(var_name.clone());
                self.variables.insert(var_name.clone(), val);
                Ok(val)
            }
            Some(Token::Number(_)) => Err(LispParseError::InvalidArgument),
            _ =>  Err(LispParseError::MissingSymbol)
        }
    }

    fn eval_unary(&mut self, iter: &mut Peekable<Iter<Token>>, operation: UnaryOp) -> Result<f64, LispParseError> {
        let mut res : Option<f64> = None;
        while iter.peek() != Some(&&Token::RightParen){
            let arg= self.get_value(iter)?;
            if res.is_none(){
                let _ = res.insert(arg);
            }
            else{
                let t = res.get_or_insert(0.0);
                match operation {
                    UnaryOp::Addition => *t+=arg,
                    UnaryOp::Subtraction => *t-=arg,
                    UnaryOp::Multiplication => *t*=arg,
                    UnaryOp::Division => *t/=arg,
                    UnaryOp::Exponential => *t = t.powf(arg),
                }
            }
        }

        match res{
            Some(res) => {
                self.parse_result = ParseResult::EvalUnary(res);
                Ok(res)
            },
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
                Ok(self.eval_parenthesis(iter)?)
            }
            Some(Token::Dollar) => {
                match &self.parse_result {
                    ParseResult::EvalUnary(val) => Ok(*val),
                    ParseResult::ParseError(e) => Err((*e).clone()),
                    _ => Err(LispParseError::InvalidArgument),
                }
            }
            Some(Token::Minus) => {
                if let Some(Token::Number(n)) = iter.next(){
                    Ok(-n)
                }
                else{
                    Err(LispParseError::InvalidArgument)
                }
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
            while iter.next_if(|t| **t == Token::RightParen).is_some(){
                expr.push(Token::RightParen);
                right_paren+=1
            }
            expr.pop();
            if right_paren != left_paren {
                return Err(LispParseError::MissingRightParen);
            }
            self.parse_result = ParseResult::InsertedFunc(fn_name.clone());
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
                self.eval_parenthesis(&mut expr.iter().peekable())
            },
            None => Err(LispParseError::InvalidFunction(function_name.clone()))
        }
    }

    fn print_function_table(&self){
        self.functions.iter()
            .for_each(|(name,fun)| println!("Name: [{}]\n{}\n",name,fun));
    }
}