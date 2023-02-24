use std::io::{self, Write};
use std::env;

mod lexer;
mod num;
mod binary_tree;
mod parser;
mod data_base;
mod operator;
mod equation;
mod solution;
mod command;
mod functions;
mod terminal;

use lexer::{Lexer, Token};
use parser::Parser;
use data_base::DataBase;
use equation::Equation;
use command::Commands;
use terminal::{TerminalController, TerminalEvent};


fn show_variable(data_base: &DataBase) -> Result<(String, String), String> {
    let string = data_base.show_variable();
    Ok((format!(""), string))
}


fn show_function(data_base: &DataBase) -> Result<(String, String), String> {
    let string = data_base.show_function()?;
    Ok((format!(""), string))
}


fn show_commands(commands: &Commands) -> (String, String) {
    let string = commands.show();
    (format!(""), string)
}


fn calculate(left_vec: Vec<Token>, data_base: &DataBase) -> Result<(String, String), String> {
    let mut parser = Parser::new(left_vec);
    let mut tree = parser.make_tree(data_base)?;

    let left_value = parser.calculation(&mut tree, data_base, None)?;
    let (result, output) = match left_value {
        Some(v) => {
            (format!("{}", v), format!("{}\n", v.to_show_value_string()))
        },
        None => {
            let s = Parser::print_tree(&tree)?;
            (s.clone(), format!("  {}\n", s))
        },
    };
    Ok((result, output))
}


fn solution_equation(left_vec: Vec<Token>, right_vec: Vec<Token>, data_base: &mut DataBase) -> Result<(String, String), String> {
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
    let solution_string = equation.solution()?;
    let result = string.clone() + solution_string.as_str();
    Ok((result, string + solution_string.as_str() + "\n"))
}


fn register(left_vec: Vec<Token>, right_vec: Vec<Token>, data_base: &mut DataBase) -> Result<(String, String), String> {
    let mut parser = Parser::new(right_vec);
    let mut tree = parser.make_tree(data_base)?;

    let right_value = parser.calculation(&mut tree, data_base, None)?;
    let right_value = match right_value {
        Some(v) => v,
        None => return Err(format!("Undefined Variables")),
    };

    let key = Parser::get_string_token_string(&left_vec[0])?;
    data_base.register_num(key, right_value)?;
    let num = data_base.get_num(&key).unwrap();
    Ok((format!("{}", num), format!("{}\n", num.to_show_value_string())))
}


fn func_register(left_vec: Vec<Token>, right_vec: Vec<Token>, data_base: &mut DataBase) -> Result<(String, String), String> {
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

    data_base.register_func(key, tree, variable.clone())?;
    let string = Parser::print_tree(&data_base.get_func(key).unwrap().0)?;
    Ok((string.clone(), format!("  {}\n", string)))
}


fn compute(code: &String, data_base: &mut DataBase, commands: &Commands) -> Result<(String, String), String> {
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
        Ok(("Unsupported format".to_string(), format!("  Unsupported format\n")))
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
            Ok((result, output)) => {
                print!("{}", output);
                result
            },
        };
        commands.push(code, result);
    }
}


fn interpreter_rich() {
    let mut terminal_controller = TerminalController::new().expect("Interpreter initialization failed");
    let mut data_base = DataBase::new();
    let mut commands = Commands::new();
    let mut command_index: i64 = -1;
    let mut temp_command = String::new();
    loop {
        match terminal_controller.run() {
            Ok(event) => match event {
                TerminalEvent::Continue => {},
                TerminalEvent::End => {
                    println!("exit");
                    break;
                },
                TerminalEvent::String(code) => {
                    command_index = -1;
                    temp_command = String::new();
                    if *code == "exit" {
                        println!("exit");
                        break;
                    }
                    let (result, output) = match compute(&code, &mut data_base, &commands) {
                        Err(e) => {
                            (e.clone(), format!("  {}\n", e))
                        },
                        Ok((result, output)) => (result, output)
                    };
                    match terminal_controller.output_string(&output) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("{}", e);
                            break
                        }
                    }
                    commands.push(*code, result);
                },
                TerminalEvent::Up(s) => {
                    if command_index == -1 {
                        temp_command = *s.clone();
                    }
                    match commands.at((command_index + 1) as usize) {
                        Some(new_s) => {
                            command_index += 1;
                            match terminal_controller.change_content(&new_s) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("{}", e);
                                    break
                                }
                            };
                        },
                        None => {
                            match terminal_controller.change_content(&s) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("{}", e);
                                    break
                                }
                            };
                        }
                    }
                }
                TerminalEvent::Down(s) => {
                    if command_index == -1 {
                        match terminal_controller.change_content(&s) {
                            Ok(_) => continue,
                            Err(e) => {
                                println!("{}", e);
                                break
                            }
                        };
                    } else if command_index == 0 {
                        match terminal_controller.change_content(&temp_command) {
                            Ok(_) => {
                                command_index = -1;
                                continue;
                            },
                            Err(e) => {
                                println!("{}", e);
                                break
                            }
                        };
                    }
                    match commands.at((command_index - 1) as usize) {
                        Some(new_s) => {
                            command_index -= 1;
                            match terminal_controller.change_content(&new_s) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("{}", e);
                                    break
                                }
                            };
                        },
                        None => {
                            match terminal_controller.change_content(&s) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("{}", e);
                                    break
                                }
                            };
                        }
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                break
            },
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] != "--rich" {
        interpreter();
    } else {
        interpreter_rich();
    }
}
