use std::io::{self, Write};

mod lexer;
mod num;
mod binary_tree;
mod parser;

use lexer::Lexer;
use parser::Parser;


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

        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        println!("{:?}", vec);

        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        println!("{:?}", tree);

        let value = match parser.calculation(&mut tree) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        println!("{:?}", value);
        // 改行は入ってこない場合もある（^Dを2回）
    }
}


fn main() {
    interpreter();
}
