use std::io::{self, Write};

mod lexer;
mod num;
mod binary_tree;
mod parser;

use lexer::{Lexer, Token};
use parser::Parser;

fn compute(code: String) -> Result<(), String> {
    let mut lexer = Lexer::new(&code);
    let vec = lexer.make_token_vec()?;
    // println!("{:?}", vec);

    let (left_vec, right_vec) = Parser::separate_equal(vec)?;

    let mut parser = Parser::new(left_vec);
    let mut tree = parser.make_tree()?;
    // println!("{:?}", tree);



    let left_value = parser.calculation(&mut tree)?;
    // println!("{:?}", left_value);

    // let mut parser = Parser::new(right_vec);
    // let mut tree = parser.make_tree()?;
    // println!("{:?}", tree);

    // let right_value = parser.calculation(&mut tree)?;
    // println!("{:?}", right_value);

    if Parser::is_question_tokens(&right_vec) {
        println!("  {}", left_value);
    }

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
