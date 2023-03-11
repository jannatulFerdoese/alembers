mod distributivity;
mod quadratic;

use alembers_ast::ast::Parser;
use alembers_lexer::lex_expression;

fn main() {
    let expression = "(1*(x^2)) - (1*x) - 1 = 0";

    let tokens = lex_expression(expression.into());
    let ast = Parser::new(tokens.as_slice()).parse().unwrap();

    let (delta, x1, x2) = quadratic::solve_quadratic(ast);

    println!("Delta: {delta}");
    println!("x1:    {x1}");
    println!("x2:    {x2}");
}
