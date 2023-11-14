use crate::parser::Parser;

mod scanner;
mod parser;


fn main() {
    let mut parser = Parser::new();
    let mut rl = rustyline::DefaultEditor::new().expect("Error failed to init readline");

    loop{
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match line.to_lowercase().as_str() {
                    "exit" => { println!("exiting"); break; }

                    _ => match parser.evaluate(&scanner::scan_tokens(line.as_str())){
                        Ok(val) => println!("{val}"),
                        Err(e) => println!("ParseError: {:?}", e),
                    }
                }

            },
            Err(_) => { println!("exiting"); break; }
        }
    }
}
