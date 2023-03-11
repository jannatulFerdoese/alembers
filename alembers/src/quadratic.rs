use std::ops::{AddAssign, SubAssign};

use alembers_ast::ast::ASTNode;

pub fn solve_quadratic(ast: ASTNode) -> (f64, f64, f64) {
    let t_ast = match ast {
        ASTNode::BinaryOp('=', left, right) => match *right {
            ASTNode::Number(n) if n == 0.0 => left,
            _ => panic!(),
        },
        _ => panic!(),
    };

    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    match *t_ast {
        ASTNode::BinaryOp(c_op, left, right) => {
            match *right {
                ASTNode::Number(n) => match c_op {
                    '+' => c.add_assign(n),
                    '-' => c.sub_assign(n),
                    _ => panic!(),
                },
                _ => panic!(),
            };

            match *left {
                ASTNode::BinaryOp(b_op, left, right) => {
                    match *left {
                        ASTNode::BinaryOp('*', left, right) => {
                            match *left {
                                ASTNode::Number(n) if n != 0.0 => a.add_assign(n),
                                _ => panic!("a must be an integer != 0"),
                            };

                            match *right {
                                ASTNode::BinaryOp('^', _, right) => match *right {
                                    ASTNode::Number(n) if n == 2.0 => {}
                                    _ => panic!(
                                        "maximum power must be 2 as it's a quadratic equation"
                                    ),
                                },
                                _ => panic!("maximum power must be 2 as it's a quadratic equation"),
                            }
                        }
                        _ => panic!(),
                    }

                    match *right {
                        ASTNode::BinaryOp('*', left, right) => {
                            match *left {
                                ASTNode::Number(n) => match b_op {
                                    '+' => b.add_assign(n),
                                    '-' => b.sub_assign(n),
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            };

                            match *right {
                                ASTNode::Identifier(_) => {}
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    let delta = b.powi(2) - (4.0 * a * c);

    if delta < 0.0 {
        panic!()
    }

    let x1 = ((b * -1.0) - delta.sqrt()) / (a * 2.0);
    let x2 = ((b * -1.0) + delta.sqrt()) / (a * 2.0);

    (delta, x1, x2)
}
