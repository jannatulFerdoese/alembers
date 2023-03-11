#[derive(PartialEq, Debug)]
pub enum Token {
    // Literals
    Float(f64),
    Indentifier(String),
    NegativeInteger(i64),
    PositiveInteger(u64),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Exp,

    // Parenthesis
    LeftParenthesis,
    RightParenthesis,

    // Equality / Inequality
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LowerThan,
    LowerThanOrEqual,
}
