#![allow(dead_code, unused)]
use alembers_ast::ast::{ASTNode, Parser};
use alembers_lexer::lex_expression;

macro_rules! expression_to_ast {
    ($expression:expr) => {
        Parser::new(lex_expression($expression.into()).as_slice())
            .parse()
            .unwrap()
    };
}

pub fn simple_distribute(ast: ASTNode) -> ASTNode {
    let (server_op, server, right_ast) = match ast {
        ASTNode::BinaryOp(server_op, left, right) if server_op == '*' => (server_op, *left, *right),
        _ => panic!(),
    };

    let (clients, op) = match right_ast {
        ASTNode::BinaryOp(op, left, right) if op == '+' || op == '-' => {
            let left_value = if let ASTNode::Identifier(_) = *left {
                left
            } else {
                panic!()
            };

            let right_value = if let ASTNode::Identifier(_) = *right {
                right
            } else {
                panic!()
            };

            ((left_value.clone(), right_value.clone()), op)
        }
        _ => panic!(),
    };

    ASTNode::BinaryOp(
        op,
        Box::new(ASTNode::BinaryOp(
            server_op,
            Box::new(server.clone()),
            clients.0,
        )),
        Box::new(ASTNode::BinaryOp(server_op, Box::new(server), clients.1)),
    )
}

#[test]
fn simple_distribute_test() {
    assert_eq!(
        simple_distribute(expression_to_ast!("a * (b + c)")).to_text(),
        "(a * b) + (a * c)"
    );

    assert_eq!(
        simple_distribute(expression_to_ast!("a * (b - c)")).to_text(),
        "(a * b) - (a * c)"
    );
}
