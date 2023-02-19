use std::io::{self, Write};

mod lexer;
mod num;
mod binary_tree;
mod parser;
mod data_base;
mod operator;
mod equation;

use lexer::Lexer;
use parser::Parser;
use data_base::DataBase;


fn compute(code: String, data_base: &mut DataBase) -> Result<(), String> {
    let mut lexer = Lexer::new(&code);
    let vec = lexer.make_token_vec()?;
    // println!("{:?}", vec);

    if Parser::is_show_variable(&vec) {
        let string = data_base.show_variable();
        print!("{}", string);
        return Ok(())
    }

    if Parser::is_show_functions(&vec) {
        let string = data_base.show_function()?;
        print!("{}", string);
        return Ok(())
    }

    let (left_vec, right_vec) = Parser::separate_equal(vec)?;

    if Parser::is_question_tokens(&right_vec) {
        let mut parser = Parser::new(left_vec);
        let mut tree = parser.make_tree(data_base)?;
        // println!("{:?}", tree);

        let left_value = parser.calculation(&mut tree, data_base, None)?;
        let left_value = match left_value {
            Some(v) => v,
            None => return Err(format!("Undefined Variables")),
        };
        println!("{}", left_value.to_show_value_string());
    } else if Parser::is_variable_register(&left_vec) {
        let mut parser = Parser::new(right_vec);
        let mut tree = parser.make_tree(data_base)?;
        // println!("{:?}", tree);

        let right_value = parser.calculation(&mut tree, data_base, None)?;
        let right_value = match right_value {
            Some(v) => v,
            None => return Err(format!("Undefined Variables")),
        };
        // println!("{:?}", right_value);

        let key = Parser::get_string_token_string(&left_vec[0])?;
        data_base.register_num(key, right_value);
        let num = data_base.get_num(&key).unwrap();
        println!("{}", num.to_show_value_string());
    } else if Parser::is_func_register(&left_vec) {
        let key = Parser::get_string_token_string(&left_vec[0])?;
        let variable = Parser::get_string_token_string(&left_vec[2])?;
        // println!("{}, {}", key, variable);

        let mut parser = Parser::new(right_vec);
        let mut tree = parser.make_tree(data_base)?;
        // println!("{:?}", tree);

        parser.calculation(&mut tree, data_base, Some((variable, None)))?;

        match Parser::check_variable_in_tree(&tree)? {
            Some(var) => {
                if var != *variable {
                    return Err(format!("{}, {}: error two variable", var, variable))
                }
            },
            None => {},
        }

        data_base.register_func(key, tree, variable.clone());
        println!("  {}", Parser::print_tree(&data_base.get_func(key).unwrap().0)?);
    } else {
        println!("  {}", "Unsupported format");
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
