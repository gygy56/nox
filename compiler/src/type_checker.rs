use crate::{ast::{BinaryOperator, Expression, Program, Statement, Type, UnaryOperator}, token::Token::Else};
use std::collections::HashMap;

pub struct TypeChecker {
    variables: HashMap<String, Type>,
    warnings: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum TypeCompatibility {
    Exact,
    Widening,
    Narrowing,
    Incompatible,
}

impl TypeChecker {
    pub  fn new() -> Self {
        Self {
            variables: HashMap::new(),
            warnings: Vec::new(),
        }
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
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
                            Expression::Nil => match expected_type {
                                Type::Optional(_) => expected_type.clone(),
                                _ => {
                                    return Err(format!(
                                        "Type error '{}': nil requires an optional type",
                                        name
                                    ));
                                }
                            },

                            _ => self.infer_expression_type(value, Some(expected_type))?,
                        }
                    }

                    None => self.infer_expression_type(value, None)?,
                };

                if let Some(expected_type) = ty {
                    match self.type_compatibility(expected_type, &actual_type) {
                        TypeCompatibility::Exact => {}

                        TypeCompatibility::Widening => {}

                        TypeCompatibility::Narrowing => {
                            self.warnings.push(format!(
                                "Warning '{}': implicit narrowing conversion from {:?} to {:?}",
                                name, actual_type, expected_type
                            ));
                        }

                        TypeCompatibility::Incompatible => {
                            return Err(format!(
                                "Type error '{}': expected {:?}, found {:?}",
                                name, expected_type, actual_type
                            ));
                        }
                    }
                }

                let variable_type = match ty {
                    Some(expected_type) => expected_type.clone(),
                    None => actual_type,
                };

                self.variables.insert(name.clone(), variable_type);

                Ok(())
            }
        }
    }

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        !matches!(
            self.type_compatibility(expected, actual),
            TypeCompatibility::Incompatible
        )
    }

    fn type_compatibility(&self, expected: &Type, actual: &Type) -> TypeCompatibility {
        if expected == actual {
            return TypeCompatibility::Exact;
        }

        match (expected, actual) {
            // ─────────────────────────────────────
            // Signed integers
            // ─────────────────────────────────────

            // Widening: smaller → larger
            (Type::I16, Type::I8)
            | (Type::I32, Type::I8 | Type::I16)
            | (Type::I64, Type::I8 | Type::I16 | Type::I32) => {
                TypeCompatibility::Widening
            }

            // Narrowing: larger → smaller
            (Type::I8, Type::I16 | Type::I32 | Type::I64)
            | (Type::I16, Type::I32 | Type::I64)
            | (Type::I32, Type::I64) => {
                TypeCompatibility::Narrowing
            }

            // ─────────────────────────────────────
            // Unsigned integers
            // ─────────────────────────────────────

            // Widening
            (Type::U16, Type::U8)
            | (Type::U32, Type::U8 | Type::U16)
            | (Type::U64, Type::U8 | Type::U16 | Type::U32) => {
                TypeCompatibility::Widening
            }

            // Narrowing
            (Type::U8, Type::U16 | Type::U32 | Type::U64)
            | (Type::U16, Type::U32 | Type::U64)
            | (Type::U32, Type::U64) => {
                TypeCompatibility::Narrowing
            }

            // ─────────────────────────────────────
            // Floating point
            // ─────────────────────────────────────

            // Widening
            (Type::F64, Type::F32) => TypeCompatibility::Widening,

            // Narrowing
            (Type::F32, Type::F64) => TypeCompatibility::Narrowing,

            // ─────────────────────────────────────
            // Optional
            // ─────────────────────────────────────

            // T → T?
            (Type::Optional(expected_inner), actual) => {
                match self.type_compatibility(expected_inner, actual) {
                    TypeCompatibility::Exact
                    | TypeCompatibility::Widening
                    | TypeCompatibility::Narrowing => TypeCompatibility::Widening,

                    TypeCompatibility::Incompatible => TypeCompatibility::Incompatible,
                }
            }

            // Tout le reste est incompatible
            _ => TypeCompatibility::Incompatible,
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
                    None => Err(format!("Unknown variable '{}'", name)),
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
                        match (&left_type, &right_type) {
                            (Type::F32, Type::F32) => Ok(Type::F32),
                            (Type::F32, Type::F64) => Ok(Type::F64),
                            (Type::F64, Type::F32) => Ok(Type::F64),
                            (Type::F64, Type::F64) => Ok(Type::F64),
                            _ => Err("Division requires floating-point operands.".to_string()),
                        }
                    }

                    BinaryOperator::IntegerDivide => {
                        match (&left_type, &right_type) {
                            (Type::I8, Type::I8)
                            | (Type::I8, Type::I16)
                            | (Type::I8, Type::I32)
                            | (Type::I8, Type::I64)
                            | (Type::I16, Type::I8)
                            | (Type::I16, Type::I16)
                            | (Type::I16, Type::I32)
                            | (Type::I16, Type::I64)
                            | (Type::I32, Type::I8)
                            | (Type::I32, Type::I16)
                            | (Type::I32, Type::I32)
                            | (Type::I32, Type::I64)
                            | (Type::I64, Type::I8)
                            | (Type::I64, Type::I16)
                            | (Type::I64, Type::I32)
                            | (Type::I64, Type::I64)
                            | (Type::U8, Type::U8)
                            | (Type::U8, Type::U16)
                            | (Type::U8, Type::U32)
                            | (Type::U8, Type::U64)
                            | (Type::U16, Type::U8)
                            | (Type::U16, Type::U16)
                            | (Type::U16, Type::U32)
                            | (Type::U16, Type::U64)
                            | (Type::U32, Type::U8)
                            | (Type::U32, Type::U16)
                            | (Type::U32, Type::U32)
                            | (Type::U32, Type::U64)
                            | (Type::U64, Type::U8)
                            | (Type::U64, Type::U16)
                            | (Type::U64, Type::U32)
                            | (Type::U64, Type::U64) => {
                                match self.numeric_result_type(&left_type, &right_type) {
                                    Some(result_type) => Ok(result_type),
                                    None => Err("Incompatible types for integer division.".to_string()),
                                }
                            }

                            _ => Err("Integer division requires integer operands.".to_string()),
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

                    BinaryOperator::BitAnd
                    | BinaryOperator::BitXor
                    | BinaryOperator::BitOr => {
                        match self.numeric_result_type(&left_type, &right_type) {
                            Some(result_type) => {
                                match result_type {
                                    Type::I8
                                    | Type::I16
                                    | Type::I32
                                    | Type::I64
                                    | Type::U8
                                    | Type::U16
                                    | Type::U32
                                    | Type::U64 => Ok(result_type),

                                    _ => Err(
                                        "Bitwise operators require integer operands."
                                            .to_string()
                                    ),
                                }
                            }

                            None => Err(
                                "Bitwise operators require compatible integer operands."
                                    .to_string()
                            ),
                        }
                    }

                    BinaryOperator::ShiftLeft
                    | BinaryOperator::ShiftRight => {
                        match (&left_type, &right_type) {
                            (
                                Type::I8 | Type::I16 | Type::I32 | Type::I64
                                | Type::U8 | Type::U16 | Type::U32 | Type::U64,
                                Type::I8 | Type::I16 | Type::I32 | Type::I64
                                | Type::U8 | Type::U16 | Type::U32 | Type::U64,
                            ) => Ok(left_type.clone()),

                            _ => Err(
                                "Shift operators require integer operands."
                                    .to_string()
                            ),
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

            Expression::Unary { operator, operand } => {
                let operand_type = self.infer_expression_type(operand, expected_type)?;

                match operator {
                    UnaryOperator::Negate => {
                        match operand_type {
                            Type::I8
                            | Type::I16
                            | Type::I32
                            | Type::I64
                            | Type::U8
                            | Type::U16
                            | Type::U32
                            | Type::U64
                            | Type::F32
                            | Type::F64 => Ok(operand_type),

                            _ => Err(
                                "Negation requires a numeric operand."
                                    .to_string()
                            ),
                        }
                    }

                    UnaryOperator::Not => {
                        if operand_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(
                                "Logical not requires a boolean operand."
                                    .to_string()
                            )
                        }
                    }
                }
            }

            Expression::MethodCall {
                object,
                method,
                arguments,
            } => {
                if !arguments.is_empty() {
                    return Err("unwrap() does not take arguments.".to_string());
                }

                let object_type = self.infer_expression_type(object, expected_type)?;

                match method.as_str() {
                    "unwrap" => {
                        match object_type {
                            Type::Optional(inner) => Ok(*inner),
                            _ => Err(
                                "unwrap() requires an optional value.".to_string()
                            ),
                        }
                    }

                    _ => Err(format!(
                        "Unknown method '{}'.",
                        method
                    )),
                }
            }
        }
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn check(source: &str) -> Result<(), String> {
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        let mut checker = TypeChecker::new();
        checker.check(&program)
    }

    #[test]
    fn accepts_basic_integer() {
        assert!(check("stck x: i32 = 42").is_ok());
    }

    #[test]
    fn accepts_inferred_integer() {
        assert!(check("stck x = 42").is_ok());
    }

    #[test]
    fn accepts_basic_float() {
        assert!(check("stck x: f64 = 3.14").is_ok());
    }

    #[test]
    fn accepts_numeric_addition() {
        assert!(check("stck x: i32 = 10 + 20").is_ok());
    }

    #[test]
    fn accepts_numeric_promotion() {
        assert!(check("stck x: i64 = 10 + 20").is_ok());
    }

    #[test]
    fn rejects_incompatible_addition() {
        assert!(check("stck x: i32 = 10 + true").is_err());
    }

    #[test]
    fn accepts_floating_division() {
        assert!(check("stck x: f64 = 10.0 / 2.0").is_ok());
    }

    #[test]
    fn rejects_integer_floating_division() {
        assert!(check("stck x: i32 = 10 / 2").is_err());
    }

    #[test]
    fn accepts_integer_division() {
        assert!(check("stck x: i32 = 10 // 2").is_ok());
    }

    #[test]
    fn rejects_integer_division_on_float() {
        assert!(check("stck x: f64 = 10.0 // 2.0").is_err());
    }

    #[test]
    fn accepts_boolean_logic() {
        assert!(check("stck x: bool = true and false").is_ok());
    }

    #[test]
    fn rejects_boolean_logic_on_integers() {
        assert!(check("stck x: bool = 10 and 20").is_err());
    }

    #[test]
    fn accepts_bitwise_operations() {
        assert!(check("stck x: i32 = 10 & 3").is_ok());
        assert!(check("stck x: i32 = 10 | 3").is_ok());
        assert!(check("stck x: i32 = 10 ^ 3").is_ok());
    }

    #[test]
    fn accepts_shift_operations() {
        assert!(check("stck x: i32 = 10 << 2").is_ok());
        assert!(check("stck x: i32 = 10 >> 2").is_ok());
    }

    #[test]
    fn accepts_comparison() {
        assert!(check("stck x: bool = 10 < 20").is_ok());
        assert!(check("stck x: bool = 10 == 20").is_ok());
    }

    #[test]
    fn accepts_optional_value() {
        assert!(check("stck x: i32? = 42").is_ok());
    }

    #[test]
    fn accepts_optional_nil() {
        assert!(check("stck x: i32? = nil").is_ok());
    }

    #[test]
    fn rejects_nil_without_optional() {
        assert!(check("stck x: i32 = nil").is_err());
    }

    #[test]
    fn rejects_optional_as_non_optional() {
        assert!(check(
            "stck a: i32? = 42
             stck b: i32 = a"
        ).is_err());
    }

    #[test]
    fn accepts_unwrap() {
        assert!(check(
            "stck a: i32? = 42
             stck b: i32 = a.unwrap()"
        ).is_ok());
    }

    #[test]
    fn rejects_unwrap_on_non_optional() {
        assert!(check("stck a: i32 = 42
                       stck b: i32 = a.unwrap()").is_err());
    }

    #[test]
    fn rejects_unknown_variable() {
        assert!(check("stck x: i32 = unknown").is_err());
    }

    #[test]
    fn accepts_unsigned_widening() {
        assert!(check(
            "stck a: u8 = 10
             stck b: u64 = a"
        ).is_ok());
    }

    #[test]
    fn accepts_unsigned_narrowing() {
        assert!(check(
            "stck a: u64 = 10
            stck b: u32 = a"
        )
        .is_ok());
    }

    #[test]
    fn accepts_signed_widening() {
        assert!(check(
            "stck a: i8 = 10
            stck b: i64 = a"
        )
        .is_ok());
    }

    #[test]
    fn accepts_signed_narrowing() {
        assert!(check(
            "stck a: i64 = 10
            stck b: i32 = a"
        )
        .is_ok());
    }

    #[test]
    fn accepts_float_widening() {
        assert!(check(
            "stck a: f32 = 10.0
            stck b: f64 = a"
        )
        .is_ok());
    }

    #[test]
    fn accepts_float_narrowing() {
        assert!(check(
            "stck a: f64 = 10.0
            stck b: f32 = a"
        )
        .is_ok());
    }
    
    #[test]
    fn widening_does_not_produce_warning() {
        let mut lexer = Lexer::new(
            "stck a: i8 = 10
            stck b: i64 = a"
                .to_string(),
        );

        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut checker = TypeChecker::new();

        assert!(checker.check(&program).is_ok());
        assert!(checker.warnings().is_empty());
    }

    #[test]
    fn narrowing_produces_warning() {
        let mut lexer = Lexer::new(
            "stck a: i64 = 10
            stck b: i32 = a"
                .to_string(),
        );

        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut checker = TypeChecker::new();

        assert!(checker.check(&program).is_ok());
        assert_eq!(checker.warnings().len(), 1);

        assert!(checker.warnings()[0].contains("narrowing"));
    }

    #[test]
    fn exact_type_does_not_produce_warning() {
        let mut lexer = Lexer::new(
            "stck a: i32 = 10
            stck b: i32 = a"
                .to_string(),
        );

        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut checker = TypeChecker::new();

        assert!(checker.check(&program).is_ok());
        assert!(checker.warnings().is_empty());
    }

}