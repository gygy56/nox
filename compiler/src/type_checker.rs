use crate::{ast::{BinaryOperator, Expression, Program, Statement, Type}, token::Token::Else};

pub struct TypeChecker;

impl TypeChecker {
    pub  fn new() -> Self {
        Self
    }

    pub fn check(&self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.check_statement(statement)?;
        }

        Ok(())
    }

    fn check_statement(&self, statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::VariableDeclaration { name, ty, value } => {
            if let Some(expected_type) = ty {
                let actual_type = match value {
                    Expression::Nil => {
                        match expected_type {
                            Type::Optional(_) => expected_type.clone(),
                            _ => {
                                return Err(format!(
                                    "Type error '{}': nil requires an optional type",
                                    name
                                ));
                            }
                        }
                    }
                    _ => self.infer_expression_type(value)?,
                };

                if !self.types_compatible(expected_type, &actual_type) {
                    return Err(format!(
                        "Type error '{}': expected {:?}, found {:?}",
                        name, expected_type, actual_type
                    ));
                }
            }

            Ok(())
        }
    }
    }

    fn infer_expression_type(&self, expression: &Expression) -> Result<Type, String> {
        match expression {
            Expression::Integer(_) => Ok(Type::I32),
            Expression::Boolean(_) => Ok(Type::Bool),
            Expression::Nil => Err("Cannot infer the type of nil".to_string()),

            Expression::Identifier(name) => {
                Err(format!("Unknow variable '{}'", name))
            }

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                
                let left_type = self.infer_expression_type(left)?;
                let right_type = self.infer_expression_type(right)?;
                
                match operator {
                    BinaryOperator::Add => {
                        if left_type == Type::I32 && right_type == Type::I32 {
                            Ok(Type::I32)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::Subtract => {
                        if left_type == Type::I32 && right_type == Type::I32 {
                            Ok(Type::I32)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::Multiply => {
                        if left_type == Type::I32 && right_type == Type::I32 {
                            Ok(Type::I32)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::Divide => {
                        if left_type == Type::I32 && right_type == Type::I32 {
                            Ok(Type::I32)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }
                    
                    BinaryOperator::Modulo => {
                        if left_type == Type::I32 && right_type == Type::I32 {
                            Ok(Type::I32)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::Greater => {
                        if left_type == right_type {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::Less => {
                        if left_type == right_type {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::GreaterEqual => {
                        if left_type == right_type {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::LessEqual => {
                        if left_type == right_type {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }
                    
                    BinaryOperator::Equal => {
                        if left_type == right_type {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::NotEqual => {
                        if left_type == right_type {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::And => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    BinaryOperator::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err("Binary operator requires operands of the same type.".to_string())
                        }
                    }

                    _ => {
                        Err("This binary operator is not implemented yet".to_string())
                    }                

                }
            }

            Expression::Unary { .. } => {
                Err("Unary expressions are not implemented yet".to_string())
            }
        }
    }

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        expected == actual
    }
}