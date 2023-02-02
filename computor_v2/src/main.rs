use std::io::{self, Write};

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

        print!("  {}", code);
    }
}


fn main() {
    interpreter();
}
