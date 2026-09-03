use crate::{ast::{BinaryOperator, Expression, Program, Statement, Type}, token::Token::Else};
use std::collections::HashMap;

pub struct TypeChecker {
    variables: HashMap<String, Type>,
}

impl TypeChecker {
    pub  fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.check_statement(statement)?;
        }

        Ok(())
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::VariableDeclaration { name, ty, value } => {
            let actual_type = match ty {
                Some(expected_type) => {
                    match value {
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
                        _ => self.infer_expression_type(value, Some(expected_type))?,
                    }
                }

                None => self.infer_expression_type(value, None)?,
            };

            if let Some(expected_type) = ty {
                if !self.types_compatible(expected_type, &actual_type) {
                    return Err(format!(
                        "Type error '{}': expected {:?}, found {:?}",
                        name, expected_type, actual_type
                    ));
                }
            }

            self.variables.insert(name.clone(), actual_type);

            Ok(())
        }
    }
}

    fn infer_expression_type(&self, expression: &Expression, expected_type: Option<&Type>) -> Result<Type, String> {
        match expression {
            Expression::Integer(_) => {
                match expected_type {
                    Some(ty) if self.is_numeric(ty) => Ok(ty.clone()),
                    _ => Ok(Type::I32)
                }
            }

            Expression::Float(_) => {
                match expected_type {
                    Some(ty) if matches!(ty, Type::F32 | Type::F64) => Ok(ty.clone()),
                    _ => Ok(Type::F64),
                }
            }

            Expression::Boolean(_) => Ok(Type::Bool),
            Expression::Nil => Err("Cannot infer the type of nil".to_string()),

            Expression::Identifier(name) => {
                match self.variables.get(name) {
                    Some(ty) => Ok(ty.clone()),
                    None => Err(format!("Unknow variable '{}'", name))
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                
                let left_type = self.infer_expression_type(left, expected_type)?;
                let right_type = self.infer_expression_type(right, expected_type)?;
                
                match operator {
                    BinaryOperator::Add => {
                        match self.numeric_result_type(&left_type, &right_type) {
                            Some(result_type) => Ok(result_type),
                            None => Err("Incompatible types for binary operator.".to_string()),
                        }
                    }
                    
                    BinaryOperator::Subtract => {
                        match self.numeric_result_type(&left_type, &right_type) {
                            Some(result_type) => Ok(result_type),
                            None => Err("Incompatible types for binary operator.".to_string()),
                        }
                    }
                    
                    BinaryOperator::Multiply => {
                        match self.numeric_result_type(&left_type, &right_type) {
                            Some(result_type) => Ok(result_type),
                            None => Err("Incompatible types for binary operator.".to_string()),
                        }
                    }

                    BinaryOperator::Divide => {
                        match self.numeric_result_type(&left_type, &right_type) {
                            Some(result_type) => Ok(result_type),
                            None => Err("Incompatible types for binary operator.".to_string()),
                        }
                    }

                    BinaryOperator::Modulo => {
                        match self.numeric_result_type(&left_type, &right_type) {
                            Some(result_type) => Ok(result_type),
                            None => Err("Incompatible types for binary operator.".to_string()),
                        }
                    }

                    BinaryOperator::Greater => {
                        if self.numeric_result_type(&left_type, &right_type).is_some() {
                            Ok(Type::Bool)
                        } else {
                            Err("Incompatible types for comparison.".to_string())
                        }
                    }

                    BinaryOperator::Less => {
                        if self.numeric_result_type(&left_type, &right_type).is_some() {
                            Ok(Type::Bool)
                        } else {
                            Err("Incompatible types for comparison.".to_string())
                        }
                    }

                    BinaryOperator::GreaterEqual => {
                        if self.numeric_result_type(&left_type, &right_type).is_some() {
                            Ok(Type::Bool)
                        } else {
                            Err("Incompatible types for comparison.".to_string())
                        }
                    }

                    BinaryOperator::LessEqual => {
                        if self.numeric_result_type(&left_type, &right_type).is_some() {
                            Ok(Type::Bool)
                        } else {
                            Err("Incompatible types for comparison.".to_string())
                        }
                    }
                    
                    BinaryOperator::Equal => {
                        if self.numeric_result_type(&left_type, &right_type).is_some() {
                            Ok(Type::Bool)
                        } else {
                            Err("Incompatible types for comparison.".to_string())
                        }
                    }

                    BinaryOperator::NotEqual => {
                        if self.numeric_result_type(&left_type, &right_type).is_some() {
                            Ok(Type::Bool)
                        } else {
                            Err("Incompatible types for comparison.".to_string())
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

    fn is_numeric(&self, ty: &Type) -> bool {
        match ty {
             Type::I8
            | Type::I16
            | Type::I32 
            | Type::I64
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::F32
            | Type::F64 => true,

            _ => false
        }
    }

    fn numeric_result_type(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            // signed
            (Type::I8, Type::I8) => Some(Type::I8),
            (Type::I8, Type::I16) | (Type::I16, Type::I8) => Some(Type::I16),
            (Type::I8, Type::I32) | (Type::I32, Type::I8) => Some(Type::I32),
            (Type::I8, Type::I64) | (Type::I64, Type::I8) => Some(Type::I64),
            (Type::I16, Type::I16) => Some(Type::I16),
            (Type::I16, Type::I32) | (Type::I32, Type::I16) => Some(Type::I32),
            (Type::I16, Type::I64) | (Type::I64, Type::I16) => Some(Type::I64),
            (Type::I32, Type::I32) => Some(Type::I32),
            (Type::I32, Type::I64) | (Type::I64, Type::I32) => Some(Type::I64),
            (Type::I64, Type::I64) => Some(Type::I64),

            // unsigned
            (Type::U8, Type::U8) => Some(Type::U8),
            (Type::U8, Type::U16) | (Type::U16, Type::U8) => Some(Type::U16),
            (Type::U8, Type::U32) | (Type::U32, Type::U8) => Some(Type::U32),
            (Type::U8, Type::U64) | (Type::U64, Type::U8) => Some(Type::U64),
            (Type::U16, Type::U16) => Some(Type::U16),
            (Type::U16, Type::U32) | (Type::U32, Type::U16) => Some(Type::U32),
            (Type::U16, Type::U64) | (Type::U64, Type::U16) => Some(Type::U64),
            (Type::U32, Type::U32) => Some(Type::U32),
            (Type::U32, Type::U64) | (Type::U64, Type::U32) => Some(Type::U64),
            (Type::U64, Type::U64) => Some(Type::U64),

            // floats
            (Type::F32, Type::F32) => Some(Type::F32),
            (Type::F32, Type::F64) | (Type::F64, Type::F32) => Some(Type::F64),
            (Type::F64, Type::F64) => Some(Type::F64),

            // incompatible families
            _ => None,
        }
    }
}