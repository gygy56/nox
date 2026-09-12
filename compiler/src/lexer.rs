use crate::token::Token;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            let current = self.source[self.position];

            if current.is_whitespace() {
                self.position += 1;
                continue;
            }

            if current.is_ascii_alphabetic() || current == '_' {
                tokens.push(self.lex_identifier());
                continue;
            }

            if current.is_ascii_digit() {
                tokens.push(self.lex_number()?);
                continue;
            }

            if current == '"' {
                tokens.push(self.lex_string()?);
                continue;
            }

            if current == '\'' {
                tokens.push(self.lex_char()?);
                continue;
            }

            let token = match current {
                '=' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        Token::EqualEqual
                    } else {
                        self.position += 1;
                        Token::Equal
                    }
                }

                '!' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        Token::NotEqual
                    } else {
                        self.position += 1;
                        Token::Not
                    }
                }

                '<' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        Token::LessEqual
                    } else if self.peek() == Some('<') {
                        self.position += 2;
                        Token::ShiftLeft
                    } else {
                        self.position += 1;
                        Token::Less
                    }
                }

                '>' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        Token::GreaterEqual
                    } else if self.peek() == Some('>') {
                        self.position += 2;
                        Token::ShiftRight
                    } else {
                        self.position += 1;
                        Token::Greater
                    }
                }

                '&' => {
                    self.position += 1;
                    Token::Ampersand
                }

                '^' => {
                    self.position += 1;
                    Token::Caret
                }

                '|' => {
                    self.position += 1;
                    Token::Pipe
                }

                '+' => {
                    self.position += 1;
                    Token::Plus
                }

                '-' => {
                    self.position += 1;
                    Token::Minus
                }

                '*' => {
                    self.position += 1;
                    Token::Star
                }

                '/' => {
                    if self.peek() == Some('/') {
                        self.position += 2;
                        Token::IntegerDivision
                    } else {
                        self.position += 1;
                        Token::Slash
                    }
                }

                '%' => {
                    self.position += 1;
                    Token::Percent
                }

                '{' => {
                    self.position += 1;
                    Token::LeftBrace
                }

                '}' => {
                    self.position += 1;
                    Token::RightBrace
                }

                '(' => {
                    self.position += 1;
                    Token::LeftParen
                }

                ')' => {
                    self.position += 1;
                    Token::RightParen
                }

                '[' => {
                    self.position += 1;
                    Token::LeftBracket
                }

                ']' => {
                    self.position += 1;
                    Token::RightBracket
                }

                ',' => {
                    self.position += 1;
                    Token::Comma
                }

                ':' => {
                    self.position += 1;
                    Token::Colon
                }

                '.' => {
                    self.position += 1;
                    Token::Dot
                }

                '?' => {
                    self.position += 1;
                    Token::Question
                }

                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at position {}",
                        current, self.position
                    ));
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);

        Ok(tokens)
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.position;

        while self.position < self.source.len()
            && (self.source[self.position].is_ascii_alphanumeric()
                || self.source[self.position] == '_')
        {
            self.position += 1;
        }

        let identifier: String = self.source[start..self.position]
            .iter()
            .collect();

        match identifier.as_str() {
            "fn" => Token::Fn,
            "stck" => Token::Stck,
            "const" => Token::Const,
            "return" => Token::Return,

            "if" => Token::If,
            "else" => Token::Else,
            "elseif" => Token::ElseIf,

            "while" => Token::While,
            "for" => Token::For,
            "loop" => Token::Loop,

            "break" => Token::Break,
            "continue" => Token::Continue,

            "true" => Token::True,
            "false" => Token::False,
            "nil" => Token::Nil,

            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,

            "i8" => Token::I8,
            "i16" => Token::I16,
            "i32" => Token::I32,
            "i64" => Token::I64,

            "u8" => Token::U8,
            "u16" => Token::U16,
            "u32" => Token::U32,
            "u64" => Token::U64,

            "f32" => Token::F32,
            "f64" => Token::F64,

            "bool" => Token::Bool,
            "char" => Token::CharType,
            "str" => Token::Str,
            "void" => Token::Void,

            _ => Token::Ident(identifier),
        }
    }

    fn lex_number(&mut self) -> Result<Token, String> {
        let start = self.position;

        while self.position < self.source.len()
            && self.source[self.position].is_ascii_digit()
        {
            self.position += 1;
        }

        if self.position < self.source.len()
            && self.source[self.position] == '.'
            && self.peek().is_some_and(|c| c.is_ascii_digit())
        {
            self.position += 1;

            while self.position < self.source.len()
                && self.source[self.position].is_ascii_digit()
            {
                self.position += 1;
            }

            let number: String = self.source[start..self.position]
                .iter()
                .collect();

            let value = number.parse::<f64>().map_err(|_| {
                format!(
                    "Invalid floating-point number '{}' at position {}",
                    number, start
                )
            })?;
            
            return Ok(Token::Float(value));
        }

        let number: String = self.source[start..self.position]
            .iter()
            .collect();

        let value = number.parse::<i64>().map_err(|_| {
            format!("Invalid integer '{}' at position {}", number, start)
        })?;

        Ok(Token::Int(value))
    }

    fn lex_string(&mut self) -> Result<Token, String> {
        self.position += 1;

        let mut value = String::new();

        while self.position < self.source.len() {
            let current = self.source[self.position];

            match current {
                '"' => {
                    self.position += 1;
                    return Ok(Token::String(value));
                }

                '\\' => {
                    self.position += 1;

                    if self.position >= self.source.len() {
                        return Err("Unterminated escape sequence".to_string());
                    }

                    let escaped = self.source[self.position];

                    let character = match escaped {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        _ => {
                            return Err(format!(
                                "Unknown escape sequence '\\{}'",
                                escaped
                            ));
                        }
                    };

                    value.push(character);
                    self.position += 1;
                }

                _ => {
                    value.push(current);
                    self.position += 1;
                }
            }
        }

        Err("Unterminated string literal".to_string())
    }

    fn lex_char(&mut self) -> Result<Token, String> {
        self.position += 1;

        if self.position >= self.source.len() {
            return Err("Unterminated character literal".to_string());
        }

        let value = if self.source[self.position] == '\\' {
            self.position += 1;

            if self.position >= self.source.len() {
                return Err("Unterminated character escape".to_string());
            }

            let escaped = self.source[self.position];

            let character = match escaped {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                _ => {
                    return Err(format!(
                        "Unknown escape sequence '\\{}'",
                        escaped
                    ));
                }
            };

            self.position += 1;
            character
        } else {
            let character = self.source[self.position];
            self.position += 1;
            character
        };

        if self.position >= self.source.len()
            || self.source[self.position] != '\''
        {
            return Err(
                "Character literal must contain exactly one character"
                    .to_string(),
            );
        }

        self.position += 1;

        Ok(Token::Char(value))
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.position + 2).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn lex_integer() {
        let mut lexer = Lexer::new("42".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Int(42),
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_float() {
        let mut lexer = Lexer::new("3.14".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Float(3.14),
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_integer_and_float() {
        let mut lexer = Lexer::new("42 3.14".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Int(42),
            Token::Float(3.14),
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_division_operators() {
        let mut lexer = Lexer::new("/ //".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Slash,
            Token::IntegerDivision,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_boolean_literals() {
        let mut lexer = Lexer::new("true false".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::True,
            Token::False,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_numeric_types() {
        let mut lexer =
            Lexer::new("i8 i16 i32 i64 u8 u16 u32 u64 f32 f64".to_string());

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::I8,
            Token::I16,
            Token::I32,
            Token::I64,
            Token::U8,
            Token::U16,
            Token::U32,
            Token::U64,
            Token::F32,
            Token::F64,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_keywords() {
        let mut lexer = Lexer::new(
            "fn stck const return if else elseif while for loop break continue"
                .to_string(),
        );

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Fn,
            Token::Stck,
            Token::Const,
            Token::Return,
            Token::If,
            Token::Else,
            Token::ElseIf,
            Token::While,
            Token::For,
            Token::Loop,
            Token::Break,
            Token::Continue,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_comparison_operators() {
        let mut lexer = Lexer::new("== != < > <= >=".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::EqualEqual,
            Token::NotEqual,
            Token::Less,
            Token::Greater,
            Token::LessEqual,
            Token::GreaterEqual,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_shift_operators() {
        let mut lexer = Lexer::new("<< >>".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::ShiftLeft,
            Token::ShiftRight,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_string() {
        let mut lexer = Lexer::new(r#""Hello""#.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::String("Hello".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_string_escape() {
        let mut lexer = Lexer::new(r#""Hello\nWorld""#.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::String("Hello\nWorld".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_char() {
        let mut lexer = Lexer::new("'A'".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Char('A'),
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_nil_and_optional() {
        let mut lexer = Lexer::new("nil ?".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Nil,
            Token::Question,
            Token::Eof,
        ]);
    }

    #[test]
    fn lex_method_call_tokens() {
        let mut lexer = Lexer::new("age.unwrap()".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::Ident("age".to_string()),
            Token::Dot,
            Token::Ident("unwrap".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Eof,
        ]);
    }

    #[test]
    fn reject_unknown_character() {
        let mut lexer = Lexer::new("@".to_string());

        assert!(lexer.tokenize().is_err());
    }
}