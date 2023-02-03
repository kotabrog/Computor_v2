use std::io::{self, Write};

mod lexer;

use lexer::Lexer;


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
        // 改行は入ってこない場合もある（^Dを2回）
    }
}


fn main() {
    interpreter();
}
