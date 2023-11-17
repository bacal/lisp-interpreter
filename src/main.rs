use rustyline::{error::ReadlineError,DefaultEditor};
use crate::parser::Parser;

mod scanner;
mod parser;

fn main() {
    let mut parser = Parser::new();
    let mut rl = DefaultEditor::new().expect("Error failed to init readline");
    loop{
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match line.to_lowercase().as_str() {
                    "exit" => { println!("exiting"); break; }
                    s if s.is_empty() => {},
                    _ => {
                        rl.add_history_entry(line.clone()).expect("lisp_interp: readline error: could not append to history");
                        parser.evaluate(&scanner::scan_tokens(line.as_str()))
                    }
                }

            },
            Err(ReadlineError::Interrupted) => {},
            Err(_) => { println!("exiting"); break; }
        }
    }
}
