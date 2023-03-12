mod solver;

use std::{fmt::Display, io::Write};

use alembers_ast::ast::Parser;
use alembers_lexer::lex_expression;

enum State {
    Solver,
    Simplifier,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strr = match self {
            Self::Solver => "solver",
            Self::Simplifier => "simplifier",
        };

        write!(f, "{}", strr)
    }
}

fn main() {
    loop {
        let mut state = State::Solver;
        let mut input = String::new();

        std::io::stdout()
            .write_all(format!("[alembers ({})]: ", state).as_bytes())
            .unwrap();

        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.as_str() {
                    "solver" => {
                        state = State::Solver;
                        continue;
                    }
                    "simplifier" => {
                        state = State::Simplifier;
                        continue;
                    }
                    _ => {}
                }

                let tokens = lex_expression(input);
                let ast = Parser::new(tokens.as_slice()).parse();

                match ast {
                    None => {
                        println!("Failed to parse AST!");
                        continue;
                    }
                    _ => {}
                }

                match state {
                    State::Solver => {
                        let solutions = solver::solver(ast.unwrap());

                        match &solutions.len() {
                            0 => println!("No solution where found."),
                            _ => {
                                for solution in solutions {
                                    println!("{}", solution.to_text());
                                }
                            }
                        }
                    }
                    State::Simplifier => {}
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
