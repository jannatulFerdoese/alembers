use alembers_common::token::Token;

pub fn lex_expression(expression: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = expression.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\n' => continue,
            '+' => tokens.push(Token::Plus),
            '-' => {
                if let Some(&('0'..='9')) = chars.peek() {
                    let mut number = String::new();
                    number.push(c);
                    while let Some(&digit) = chars.peek() {
                        match digit {
                            '0'..='9' | '.' => {
                                number.push(digit);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    let value = number.parse().unwrap();
                    tokens.push(if c == '-' {
                        Token::NegativeInteger(value)
                    } else {
                        Token::Float(value as f64)
                    });
                } else {
                    tokens.push(Token::Minus)
                }
            }

            '^' => tokens.push(Token::Exp),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            '=' => tokens.push(Token::Equal),
            '>' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterThanOrEqual);
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '<' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LowerThanOrEqual);
                } else {
                    tokens.push(Token::LowerThan);
                }
            }
            '0'..='9' => {
                let mut number = String::new();
                let mut found_dot = false;
                number.push(c);
                while let Some(&digit) = chars.peek() {
                    match digit {
                        '.' => {
                            found_dot = true;
                            number.push(digit);
                            chars.next();
                        }
                        '0'..='9' => {
                            number.push(digit);
                            chars.next();
                        }
                        _ => break,
                    }
                }

                tokens.push(if found_dot {
                    Token::Float(number.parse().unwrap())
                } else {
                    Token::PositiveInteger(number.parse().unwrap())
                })
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            identifier.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Indentifier(identifier));
            }
            _ => panic!(),
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_expression() {
        assert_eq!(
            lex_expression(String::from("1 + 2")),
            vec![
                Token::PositiveInteger(1),
                Token::Plus,
                Token::PositiveInteger(2)
            ]
        );

        assert_eq!(
            lex_expression(String::from("3.14 * (2 - 1)")),
            vec![
                Token::Float(3.14),
                Token::Multiply,
                Token::LeftParenthesis,
                Token::PositiveInteger(2),
                Token::Minus,
                Token::PositiveInteger(1),
                Token::RightParenthesis
            ]
        );
        assert_eq!(
            lex_expression(String::from("x = 10")),
            vec![
                Token::Indentifier(String::from("x")),
                Token::Equal,
                Token::PositiveInteger(10)
            ]
        );
    }
}
