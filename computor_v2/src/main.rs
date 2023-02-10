use std::io::{self, Write};

mod lexer;
mod num;
mod binary_tree;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn compute(code: String) -> Result<(), String> {
    let mut lexer = Lexer::new(&code);
    let vec = lexer.make_token_vec()?;
    println!("{:?}", vec);

    let mut parser = Parser::new(vec);
    let mut tree = parser.make_tree()?;
    println!("{:?}", tree);

    let value = parser.calculation(&mut tree)?;
    println!("{:?}", value);
    Ok(())
}

fn interpreter() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("Failed to read line");

        if code == "exit\n" || code == "" {
            println!("exit");
            break;
        }

        match compute(code) {
            Err(e) => println!("  {}", e),
            _ => {},
        }
    }
}


fn main() {
    interpreter();
}
