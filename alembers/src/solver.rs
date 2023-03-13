#![allow(unused)]
use alembers::factorize;
use alembers::functions::linear::linear_function;
use alembers_ast::ast::ASTNode;

pub fn solver(ast: ASTNode) -> Vec<ASTNode> {
    let mut solutions = vec![];

    let (left_member, right_member) = match ast {
        ASTNode::BinaryOp('=', left_member, right_member) => (*left_member, *right_member),
        _ => return vec![],
    };

    match right_member {
        ASTNode::Number(x) if x == 0.0 => {
            if let Some(fun) = linear_function(left_member.clone()) {
                solutions.push(ASTNode::BinaryOp(
                    '=',
                    Box::new(ASTNode::Identifier(fun.clone().variable)),
                    Box::new(ASTNode::Number(fun.zero())),
                ))
            }
        }
        _ => {}
    }

    match left_member {
        ASTNode::Number(x) if x == 0.0 => {
            if let Some(fun) = linear_function(right_member) {
                solutions.push(ASTNode::BinaryOp(
                    '=',
                    Box::new(ASTNode::Identifier(fun.variable)),
                    Box::new(ASTNode::Number((fun.b.unwrap_or(0.0) * -1.0) / fun.a)),
                ))
            }
        }
        _ => {}
    }

    solutions
}
