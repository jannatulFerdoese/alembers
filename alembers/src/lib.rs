use alembers_ast::ast::ASTNode;

pub fn simple_distribute(ast: ASTNode) -> Option<ASTNode> {
    let (server_op, server, right_ast) = match ast {
        ASTNode::BinaryOp(server_op, left, right) if server_op == '*' => (server_op, *left, *right),
        _ => return None,
    };

    let (clients, op) = match right_ast {
        ASTNode::BinaryOp(op, left, right) if op == '+' || op == '-' => {
            let left_value = if let ASTNode::Identifier(_) = *left {
                left
            } else {
                return None;
            };

            let right_value = if let ASTNode::Identifier(_) = *right {
                right
            } else {
                return None;
            };

            ((left_value.clone(), right_value.clone()), op)
        }
        _ => return None,
    };

    Some(ASTNode::BinaryOp(
        op,
        Box::new(ASTNode::BinaryOp(
            server_op,
            Box::new(server.clone()),
            clients.0,
        )),
        Box::new(ASTNode::BinaryOp(server_op, Box::new(server), clients.1)),
    ))
}

pub fn a_squared_minus_b_squared(ast: ASTNode) -> Option<ASTNode> {
    match ast {
        ASTNode::BinaryOp('-', a_squared, b_squared) => {
            let a = match *a_squared {
                ASTNode::BinaryOp('^', a, exp) => match *exp {
                    ASTNode::Number(n) if n == 2.0 => a,
                    _ => return None,
                },
                _ => return None,
            };

            let b = match *b_squared {
                ASTNode::BinaryOp('^', b, exp) => match *exp {
                    ASTNode::Number(n) if n == 2.0 => b,
                    _ => return None,
                },
                _ => return None,
            };

            Some(ASTNode::BinaryOp(
                '*',
                Box::new(ASTNode::BinaryOp('+', a.clone(), b.clone())),
                Box::new(ASTNode::BinaryOp('-', a, b)),
            ))
        }
        _ => return None,
    }
}

/// (a+-b)^2 = a^2 +- 2ab + b^2
pub fn a_plus_minus_b_squared(ast: ASTNode) -> Option<ASTNode> {
    match ast {
        ASTNode::BinaryOp('^', a_plus_b, exp) => {
            match *exp {
                ASTNode::Number(n) if n == 2.0 => {}
                _ => return None,
            }

            let (op, a, b) = match *a_plus_b {
                ASTNode::BinaryOp('+', a, b) => ('+', a, b),
                ASTNode::BinaryOp('-', a, b) => ('-', a, b),
                _ => return None,
            };

            Some(ASTNode::BinaryOp(
                '+',
                Box::new(ASTNode::BinaryOp('^', a.clone(), exp.clone())),
                Box::new(ASTNode::BinaryOp(
                    op,
                    Box::new(ASTNode::BinaryOp('^', b.clone(), exp)),
                    Box::new(ASTNode::BinaryOp(
                        '*',
                        Box::new(ASTNode::Number(2.0)),
                        Box::new(ASTNode::BinaryOp('*', a, b)),
                    )),
                )),
            ))
        }
        _ => return None,
    }
}

/// (b^m)^n = b^(m*n)
pub fn b_exp_m_exp_n(ast: ASTNode) -> Option<ASTNode> {
    match ast {
        ASTNode::BinaryOp('^', base_b_m, exp_n) => {
            let (base_b, exp_m) = match *base_b_m {
                ASTNode::BinaryOp('^', base_b, exp_m) => (base_b, exp_m),
                _ => return None,
            };

            Some(ASTNode::BinaryOp(
                '^',
                base_b,
                Box::new(ASTNode::BinaryOp('*', exp_m, exp_n)),
            ))
        }
        _ => return None,
    }
}

/// b^(m+n) = b^m * b^n
pub fn b_exp_m_plus_n(ast: ASTNode) -> Option<ASTNode> {
    match ast {
        ASTNode::BinaryOp('^', base, exp) => {
            let (exp_left, exp_right) = match *exp {
                ASTNode::BinaryOp('+', left, right) => (left, right),
                _ => return None,
            };

            Some(ASTNode::BinaryOp(
                '*',
                Box::new(ASTNode::BinaryOp('^', base.clone(), exp_left)),
                Box::new(ASTNode::BinaryOp('^', base, exp_right)),
            ))
        }
        _ => return None,
    }
}
