use core::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Eof,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Number(f64),
    None,
}

pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Literal,
    #[allow(dead_code)]
    pub(crate) line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self.token_type {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::Star => "STAR",
            TokenType::Dot => "DOT",
            TokenType::Comma => "COMMA",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Slash => "SLASH",
            TokenType::Eof => "EOF",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::AND => "AND",
            TokenType::CLASS => "CLASS",
            TokenType::ELSE => "ELSE",
            TokenType::FALSE => "FALSE",
            TokenType::FOR => "FOR",
            TokenType::FUN => "FUN",
            TokenType::IF => "IF",
            TokenType::NIL => "NIL",
            TokenType::OR => "OR",
            TokenType::PRINT => "PRINT",
            TokenType::RETURN => "RETURN",
            TokenType::SUPER => "SUPER",
            TokenType::THIS => "THIS",
            TokenType::TRUE => "TRUE",
            TokenType::VAR => "VAR",
            TokenType::WHILE => "WHILE",
        };

        let literal_str = match &self.literal {
            Literal::Str(s) => format!("{s}"),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{n}.0")
                } else {
                    format!("{n}")
                }
            }
            Literal::None => format!("null"),
        };

        write!(f, "{} {} {}", type_str, self.lexeme, literal_str)
    }
}
