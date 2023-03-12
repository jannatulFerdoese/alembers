use alembers_ast::ast::ASTNode;

pub fn solver(ast: ASTNode) -> Vec<ASTNode> {
    let mut solutions = vec![];

    match ast {
        ASTNode::BinaryOp('=', left_member, right_member) => {
            match *left_member.clone() {
                ASTNode::BinaryOp('*', l, r) => {
                    match (*l.clone(), *r) {
                        (ASTNode::Number(a), ASTNode::Identifier(x))
                        | (ASTNode::Identifier(x), ASTNode::Number(a)) => {
                            match *right_member {
                                ASTNode::Number(b) => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '/',
                                        Box::new(ASTNode::Number(b)),
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),

                                _ => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '/',
                                        right_member,
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),
                            };
                        }
                        _ => {}
                    };
                }

                ASTNode::BinaryOp('/', l, r) => {
                    match (*l.clone(), *r) {
                        (ASTNode::Number(a), ASTNode::Identifier(x))
                        | (ASTNode::Identifier(x), ASTNode::Number(a)) => {
                            match *right_member {
                                ASTNode::Number(b) => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '*',
                                        Box::new(ASTNode::Number(b)),
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),

                                _ => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '*',
                                        right_member,
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),
                            };
                        }
                        _ => {}
                    };
                }

                ASTNode::BinaryOp('+', l, r) => {
                    match (*l.clone(), *r) {
                        (ASTNode::Number(a), ASTNode::Identifier(x))
                        | (ASTNode::Identifier(x), ASTNode::Number(a)) => {
                            match *right_member {
                                ASTNode::Number(b) => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '-',
                                        Box::new(ASTNode::Number(b)),
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),

                                _ => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '-',
                                        right_member,
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),
                            };
                        }
                        _ => {}
                    };
                }

                ASTNode::BinaryOp('-', l, r) => {
                    match (*l.clone(), *r) {
                        (ASTNode::Number(a), ASTNode::Identifier(x))
                        | (ASTNode::Identifier(x), ASTNode::Number(a)) => {
                            match *right_member {
                                ASTNode::Number(b) => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '+',
                                        Box::new(ASTNode::Number(b)),
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),

                                _ => solutions.push(ASTNode::BinaryOp(
                                    '=',
                                    Box::new(ASTNode::Identifier(x)),
                                    Box::new(ASTNode::BinaryOp(
                                        '+',
                                        right_member,
                                        Box::new(ASTNode::Number(a)),
                                    )),
                                )),
                            };
                        }
                        _ => {}
                    };
                }
                _ => {}
            };
        }
        _ => println!("Input was not an equation!"),
    }

    solutions
}
