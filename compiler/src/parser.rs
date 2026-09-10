use std::fmt::Arguments;

use crate::ast::{
    BinaryOperator,
    Expression,
    Program,
    Statement,
    Type,
    UnaryOperator,
};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.check(&Token::Eof) {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current() {
            Some(Token::Stck) => self.parse_variable_declaration(),

            Some(token) => Err(format!(
                "Expected a statement, found {:?}",
                token
            )),

            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        self.advance();

        let name = match self.current() {
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                name
            }

            Some(token) => {
                return Err(format!(
                    "Expected identifier after 'stck', found {:?}",
                    token
                ));
            }

            None => {
                return Err(
                    "Expected identifier after 'stck', found end of input"
                        .to_string(),
                );
            }
        };

        let ty = if self.check(&Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        if !self.check(&Token::Equal) {
            return Err("Expected '=' in variable declaration".to_string());
        }

        self.advance();

        let value = self.parse_expression()?;

        Ok(Statement::VariableDeclaration {
            name,
            ty,
            value,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let mut ty = match self.current() {
            Some(Token::I8) => {
                self.advance();
                Type::I8
            }

            Some(Token::I16) => {
                self.advance();
                Type::I16
            }

            Some(Token::I32) => {
                self.advance();
                Type::I32
            }

            Some(Token::I64) => {
                self.advance();
                Type::I64
            }

            Some(Token::U8) => {
                self.advance();
                Type::U8
            }

            Some(Token::U16) => {
                self.advance();
                Type::U16
            }

            Some(Token::U32) => {
                self.advance();
                Type::U32
            }

            Some(Token::U64) => {
                self.advance();
                Type::U64
            }

            Some(Token::F32) => {
                self.advance();
                Type::F32
            }

            Some(Token::F64) => {
                self.advance();
                Type::F64
            }

            Some(Token::Bool) => {
                self.advance();
                Type::Bool
            }

            Some(token) => {
                return Err(format!("Unknown type {:?}", token));
            }
            
            None => {
                return Err("Expected a type, found end of input".to_string());
            }
        };

        if self.check(&Token::Question) {
            self.advance();
            ty = Type::Optional(Box::new(ty));
        }

        Ok(ty)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {    
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_and()?;

        while self.check(&Token::Or) {
            self.advance();

            let right = self.parse_and()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_shift()?;

        while self.check(&Token::And) {
            self.advance();

            let right = self.parse_shift()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_bit_or()?;

        loop {
            let operator = match self.current() {
                Some(Token::ShiftLeft) => BinaryOperator::ShiftLeft,
                Some(Token::ShiftRight) => BinaryOperator::ShiftRight,
                _ => break,
            };

            self.advance();

            let right = self.parse_bit_or()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bit_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_bit_xor()?;

        while self.check(&Token::Pipe) {
            self.advance();

            let right = self.parse_bit_xor()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::BitOr,
                right: Box::new(right),
            };
        } 

        Ok(left)
    }

    fn parse_bit_xor(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_bit_and()?;

        while self.check(&Token::Caret) {
            self.advance();

            let right = self.parse_bit_and()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::BitXor,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bit_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;

        while self.check(&Token::Ampersand) {
            self.advance();

            let right = self.parse_comparison()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::BitAnd,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_additive()?;

        loop {
            let operator = match self.current() {
                Some(Token::EqualEqual) => BinaryOperator::Equal,
                Some(Token::NotEqual) => BinaryOperator::NotEqual,
                Some(Token::Less) => BinaryOperator::Less,
                Some(Token::Greater) => BinaryOperator::Greater,
                Some(Token::LessEqual) => BinaryOperator::LessEqual,
                Some(Token::GreaterEqual) => BinaryOperator::GreaterEqual,
                _ => break,
            };

            self.advance();

            let right = self.parse_additive()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let operator = match self.current() {
                Some(Token::Plus) => BinaryOperator::Add,
                Some(Token::Minus) => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.parse_multiplicative()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;

        loop {
            let operator = match self.current() {
                Some(Token::Star) => BinaryOperator::Multiply,
                Some(Token::Slash) => BinaryOperator::Divide,
                Some(Token::IntegerDivision) => BinaryOperator::IntegerDivide,
                Some(Token::Percent) => BinaryOperator::Modulo,
                _ => break,
            };

            self.advance();

            let right = self.parse_unary()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        match self.current() {
            Some(Token::Minus) => {
                self.advance();

                let operand = self.parse_unary()?;

                Ok(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    operand: Box::new(operand),
                })
            }

            Some(Token::Not) => {
                self.advance();

                let operand = self.parse_unary()?;

                Ok(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: Box::new(operand),
                })
            }

            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_primary()?;
        loop {
            if self.check(&Token::Dot) {
                self.advance();

                let method= match self.current() {
                    Some(Token::Ident(name)) => {
                        let name = name.clone();
                        self.advance();
                        name
                    }

                    Some(token) => {
                        return Err(format!(
                            "Expected method name after '.', found {:?}",
                            token
                        ));
                    }

                    None => {
                        return Err(
                            "Expected method name after '.', found end of input"
                                .to_string()
                        );
                    }
                };

                if !self.check(&Token::LeftParen) {
                    return Err(
                        "Expected '(' after method name".to_string()
                    );
                }

                self.advance();

                let mut arguments = Vec::new();

                if !self.check(&Token::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);

                        if self.check(&Token::RightParen) {
                            break;
                        }

                        return Err(
                            "Expected '(' after method arguments".to_string()
                        );
                    }
                }

                self.advance();

                expression = Expression::MethodCall {
                    object: Box::new(expression),
                    method,
                    arguments,
                };
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current() {
            Some(Token::Int(value)) => {
                let value = *value;
                self.advance();

                Ok(Expression::Integer(value))
            }

            Some(Token::Float(value)) => {
                let value = *value;
                self.advance();
                
                Ok(Expression::Float(value))
            }

            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();

                Ok(Expression::Identifier(name))
            }
            
            Some(Token::True) => {
                self.advance();
                Ok(Expression::Boolean(true))
            }

            Some(Token::False) => {
                self.advance();
                Ok(Expression::Boolean(false))
            }

            Some(Token::Nil) => {
                self.advance();
                Ok(Expression::Nil)
            }

            Some(Token::LeftParen) => {
                self.advance();

                let expression = self.parse_expression()?;

                if !self.check(&Token::RightParen) {
                    return Err("Expected ')'".to_string());
                }

                self.advance();

                Ok(expression)
            }
            


            Some(token) => Err(format!(
                "Expected an expression, found {:?}",
                token
            )),

            None => Err(
                "Expected an expression, found end of input".to_string()
            ),
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn check(&self, token: &Token) -> bool {
        self.current() == Some(token)
    }
}