use alembers_ast::ast::ASTNode;

pub fn b_exp_m_exp_n(ast: ASTNode) -> ASTNode {
    match ast {
        ASTNode::BinaryOp('^', base_b_m, exp_n) => {
            let (base_b, exp_m) = match *base_b_m {
                ASTNode::BinaryOp('^', base_b, exp_m) => (base_b, exp_m),
                _ => panic!(),
            };

            ASTNode::BinaryOp('^', base_b, Box::new(ASTNode::BinaryOp('*', exp_m, exp_n)))
        }
        _ => panic!(),
    }
}

pub fn b_exp_m_plus_n(ast: ASTNode) -> ASTNode {
    match ast {
        ASTNode::BinaryOp('^', base, exp) => {
            let (exp_left, exp_right) = match *exp {
                ASTNode::BinaryOp('+', left, right) => (left, right),
                _ => panic!(),
            };

            ASTNode::BinaryOp(
                '*',
                Box::new(ASTNode::BinaryOp('^', base.clone(), exp_left)),
                Box::new(ASTNode::BinaryOp('^', base, exp_right)),
            )
        }
        _ => panic!(),
    }
}

#[test]
fn b_exp_m_plus_n_test() {}
