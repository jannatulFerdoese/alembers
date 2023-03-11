use alembers_ast::ast::Parser;
use alembers_lexer::lex_expression;

fn main() {
    let ast = Parser::new(lex_expression("a * (b + c)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::simple_distribute(ast).unwrap().to_text()
    );

    let ast = Parser::new(lex_expression("a * (b - c)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::simple_distribute(ast).unwrap().to_text()
    );

    let ast = Parser::new(lex_expression("b^((2*m)+n)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::b_exp_m_plus_n(ast).unwrap().to_text()
    );

    let ast = Parser::new(lex_expression("(b^m)^n".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::b_exp_m_exp_n(ast).unwrap().to_text()
    );

    let ast = Parser::new(lex_expression("(a+b)^2".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::a_plus_minus_b_squared(ast).unwrap().to_text()
    );

    let ast = Parser::new(lex_expression("(a-b)^2".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::a_plus_minus_b_squared(ast).unwrap().to_text()
    );

    let ast = Parser::new(lex_expression("(a^2) - (b^2)".into()).as_slice())
        .parse()
        .unwrap();

    println!(
        "{} <=> {}\n",
        ast.to_text(),
        alembers::a_squared_minus_b_squared(ast).unwrap().to_text()
    );
}
