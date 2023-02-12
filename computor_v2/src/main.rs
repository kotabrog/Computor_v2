use std::io::{self, Write};

mod lexer;
mod num;
mod binary_tree;
mod parser;
mod data_base;
mod operator;

use lexer::{Lexer, Token};
use parser::Parser;
use data_base::DataBase;


fn compute(code: String, data_base: &mut DataBase) -> Result<(), String> {
    let mut lexer = Lexer::new(&code);
    let vec = lexer.make_token_vec()?;
    // println!("{:?}", vec);

    let (left_vec, right_vec) = Parser::separate_equal(vec)?;

    if Parser::is_variable_register(&left_vec) {
        let mut parser = Parser::new(right_vec);
        let mut tree = parser.make_tree()?;
        // println!("{:?}", tree);

        let right_value = parser.calculation(&mut tree, data_base)?;
        // println!("{:?}", right_value);

        let key = Parser::get_string_token_string(&left_vec[0])?;
        data_base.register(key, right_value);
        println!("  {}", data_base.get(&key).unwrap());
    } else if Parser::is_question_tokens(&right_vec){
        let mut parser = Parser::new(left_vec);
        let mut tree = parser.make_tree()?;
        // println!("{:?}", tree);

        let left_value = parser.calculation(&mut tree, data_base)?;
        // println!("{:?}", left_value);

        println!("  {}", left_value);
    }

    // println!("{:?}", data_base);
    Ok(())
}

fn interpreter() {
    let mut data_base = DataBase::new();
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

        match compute(code, &mut data_base) {
            Err(e) => println!("  {}", e),
            _ => {},
        }
    }
}


fn main() {
    interpreter();
}
