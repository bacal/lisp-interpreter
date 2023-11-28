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
                    "ftab" => parser.print_function_table(),
                    "vars" => parser.print_variables(),
                    "help" => print_help(),
                    "clear" => rl.clear_screen().expect("lisp_interp: failed to clear screen"),
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

fn print_help() {
    println!("Builtins");
    println!("ftab - print function table");
    println!("vars - print stored variables");
    println!("clear - clears screen");
    println!("help - print help");
    println!("exit - close interpreter");
}
