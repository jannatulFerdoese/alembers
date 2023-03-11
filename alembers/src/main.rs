mod distributivity;
mod identities;
mod quadratic;

use alembers_ast::ast::Parser;
use alembers_lexer::lex_expression;

fn main() {
    let ast = Parser::new(lex_expression("a * (b + c)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}",
        ast.to_text(),
        distributivity::simple_distribute(ast).to_text()
    );

    let ast = Parser::new(lex_expression("a * (b - c)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}",
        ast.to_text(),
        distributivity::simple_distribute(ast).to_text()
    );

    let ast = Parser::new(lex_expression("b^((2*m)+n)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}",
        ast.to_text(),
        identities::b_exp_m_plus_n(ast).to_text()
    );

    let ast = Parser::new(lex_expression("(b^m)^n".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}",
        ast.to_text(),
        identities::b_exp_m_exp_n(ast).to_text()
    );
}
