use std::io::{self, Write};

mod lexer;
mod num;
mod binary_tree;
mod parser;
mod data_base;
mod operator;
mod equation;
mod solution;
mod command;

use lexer::{Lexer, Token};
use parser::Parser;
use data_base::DataBase;
use equation::Equation;
use command::Commands;


fn show_variable(data_base: &DataBase) -> Result<String, String> {
    let string = data_base.show_variable();
    print!("{}", string);
    Ok(format!(""))
}


fn show_function(data_base: &DataBase) -> Result<String, String> {
    let string = data_base.show_function()?;
    print!("{}", string);
    Ok(format!(""))
}


fn show_commands(commands: &Commands) -> String {
    let string = commands.show();
    print!("{}", string);
    format!("")
}


fn calculate(left_vec: Vec<Token>, data_base: &DataBase) -> Result<String, String> {
    let mut parser = Parser::new(left_vec);
    let mut tree = parser.make_tree(data_base)?;

    let left_value = parser.calculation(&mut tree, data_base, None)?;
    let left_value = match left_value {
        Some(v) => v,
        None => return Err(format!("Undefined Variables")),
    };
    println!("{}", left_value.to_show_value_string());
    Ok(format!("{}", left_value))
}


fn solution_equation(left_vec: Vec<Token>, right_vec: Vec<Token>, data_base: &mut DataBase) -> Result<String, String> {
    let mut parser = Parser::new(left_vec);
    let mut left_tree = parser.make_tree(data_base)?;
    parser.calculation(&mut left_tree, data_base, None)?;

    let mut right_vec = right_vec;
    right_vec.pop();
    let mut parser = Parser::new(right_vec);
    let mut right_tree = parser.make_tree(data_base)?;
    parser.calculation(&mut right_tree, data_base, None)?;

    let mut equation = Equation::new();
    equation.make_equation(&left_tree, &right_tree)?;
    let string = format!("  {} = 0\n", equation.to_string()?);
    print!("{}", string);
    let solution_string = equation.solution()?;
    println!("{}", solution_string);
    Ok(string + solution_string.as_str())
}


fn register(left_vec: Vec<Token>, right_vec: Vec<Token>, data_base: &mut DataBase) -> Result<String, String> {
    let mut parser = Parser::new(right_vec);
    let mut tree = parser.make_tree(data_base)?;

    let right_value = parser.calculation(&mut tree, data_base, None)?;
    let right_value = match right_value {
        Some(v) => v,
        None => return Err(format!("Undefined Variables")),
    };

    let key = Parser::get_string_token_string(&left_vec[0])?;
    data_base.register_num(key, right_value);
    let num = data_base.get_num(&key).unwrap();
    println!("{}", num.to_show_value_string());
    Ok(format!("{}", num))
}


fn func_register(left_vec: Vec<Token>, right_vec: Vec<Token>, data_base: &mut DataBase) -> Result<String, String> {
    let key = Parser::get_string_token_string(&left_vec[0])?;
    let variable = Parser::get_string_token_string(&left_vec[2])?;

    let mut parser = Parser::new(right_vec);
    let mut tree = parser.make_tree(data_base)?;

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
    let string = Parser::print_tree(&data_base.get_func(key).unwrap().0)?;
    println!("  {}", string);
    Ok(string)
}


fn compute(code: &String, data_base: &mut DataBase, commands: &Commands) -> Result<String, String> {
    let mut lexer = Lexer::new(&code);
    let vec = lexer.make_token_vec()?;

    if Parser::is_show_variable(&vec) {
        return show_variable(&data_base)
    } else if Parser::is_show_functions(&vec) {
        return show_function(&data_base)
    } else if Parser::is_show_commands(&vec) {
        return Ok(show_commands(&commands))
    }

    let (left_vec, right_vec) = Parser::separate_equal(vec)?;

    if Parser::is_question_tokens(&right_vec) {
        calculate(left_vec, &data_base)
    } else if Parser::is_solution_equation(&right_vec) {
        solution_equation(left_vec, right_vec, data_base)
    } else if Parser::is_variable_register(&left_vec) {
        register(left_vec, right_vec, data_base)
    } else if Parser::is_func_register(&left_vec) {
        func_register(left_vec, right_vec, data_base)
    } else {
        println!("  {}", "Unsupported format");
        Ok("Unsupported format".to_string())
    }
}


fn interpreter() {
    let mut data_base = DataBase::new();
    let mut commands = Commands::new();
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

        let result = match compute(&code, &mut data_base, &commands) {
            Err(e) => {
                println!("  {}", e);
                e
            },
            Ok(s) => s,
        };
        commands.push(code, result);
    }
}


fn main() {
    interpreter();
}
