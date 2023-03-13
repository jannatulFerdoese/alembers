use alembers_ast::ast::ASTNode;

#[derive(Debug, Clone)]
pub struct LinearFunction {
    pub a: f64,
    pub b: Option<f64>,
    pub variable: String,
}

impl LinearFunction {
    pub fn zero(&self) -> f64 {
        (self.b.unwrap_or(0.0) * -1.0) / self.a
    }
}

pub fn linear_function(ast: ASTNode) -> Option<LinearFunction> {
    if let ASTNode::BinaryOp(op, left, right) = ast {
        if op != '+' && op != '-' && op == '*' {
            return None;
        }

        if op == '*' {
            let (a, variable) = match (*left, *right) {
                (ASTNode::Number(a), ASTNode::Identifier(variable)) => (a, variable),
                (ASTNode::Identifier(variable), ASTNode::Number(a)) => (a, variable),
                _ => return None,
            };

            Some(LinearFunction {
                a,
                variable,
                b: None,
            })
        } else {
            let (b, a_ast) = match (*left, *right) {
                (ASTNode::Number(b), a) => (b, a),
                (a, ASTNode::Number(b)) => (b, a),
                _ => return None,
            };

            let (a, variable) = match a_ast {
                ASTNode::BinaryOp('*', left, right) => match (*left, *right) {
                    (ASTNode::Number(a), ASTNode::Identifier(variable)) => (a, variable),
                    (ASTNode::Identifier(variable), ASTNode::Number(a)) => (a, variable),
                    _ => return None,
                },
                _ => return None,
            };

            match op {
                '+' => Some(LinearFunction {
                    a,
                    variable,
                    b: Some(b),
                }),
                '-' => Some(LinearFunction {
                    a,
                    variable,
                    b: Some(b * -1.0),
                }),
                _ => None,
            }
        }
    } else {
        None
    }
}
