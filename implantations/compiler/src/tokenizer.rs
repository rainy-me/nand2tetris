use std::str::Chars;
use std::{fmt, path::PathBuf};

// impl std::str::FromStr for Keyword {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "class" => Ok(Keyword::Class),
//             "constructor" => Ok(Keyword::Constructor),
//             "function" => Ok(Keyword::Function),
//             "method" => Ok(Keyword::Method),
//             "field" => Ok(Keyword::Field),
//             "static" => Ok(Keyword::Static),
//             "var" => Ok(Keyword::Var),
//             "int" => Ok(Keyword::Int),
//             "char" => Ok(Keyword::Char),
//             "boolean" => Ok(Keyword::Boolean),
//             "void" => Ok(Keyword::Void),
//             "true" => Ok(Keyword::True),
//             "false" => Ok(Keyword::False),
//             "null" => Ok(Keyword::Null),
//             "this" => Ok(Keyword::This),
//             "let" => Ok(Keyword::Let),
//             "do" => Ok(Keyword::Do),
//             "if" => Ok(Keyword::If),
//             "else" => Ok(Keyword::Else),
//             "while" => Ok(Keyword::While),
//             "return" => Ok(Keyword::Return),
//             _ => Err(()),
//         }
//     }
// }

// impl std::convert::TryFrom<char> for Symbol {
//     type Error = ();
//
//     fn try_from(s: char) -> Result<Self, Self::Error> {
//         match s {
//             '{' => Ok(Symbol::LBrace),
//             '}' => Ok(Symbol::RBrace),
//             '(' => Ok(Symbol::LParen),
//             ')' => Ok(Symbol::RParen),
//             '[' => Ok(Symbol::LBracket),
//             ']' => Ok(Symbol::RBracket),
//             '.' => Ok(Symbol::Dot),
//             ',' => Ok(Symbol::Comma),
//             ';' => Ok(Symbol::Semicolon),
//             '+' => Ok(Symbol::Plus),
//             '-' => Ok(Symbol::Minus),
//             '*' => Ok(Symbol::Asterisk),
//             '/' => Ok(Symbol::Slash),
//             '&' => Ok(Symbol::And),
//             '<' => Ok(Symbol::LT),
//             '>' => Ok(Symbol::GT),
//             '=' => Ok(Symbol::EQ),
//             '~' => Ok(Symbol::Not),
//             _ => Err(()),
//         }
//     }
// }

pub struct Token<'a> {
    kind: TokenKind,
    literal: Literal<'a>,
}
enum Literal<'a> {
    Integer(u16),
    String(&'a str),
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(val) => write!(f, "{}", val),
            Literal::String(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Comment,
    IntegerConstant,
    StringConstant,
    Identifier,
    Trivia,
    // Keyword
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    // Symbol
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    LT,
    GT,
    EQ,
    Not,
}

use TokenKind::*;

impl TokenKind {
    fn as_str(&self) -> Option<&'static str> {
        match self {
            Class | Constructor | Function | Method | Field | Static | Var | Int | Char
            | Boolean | Void | True | False | Null | This | Let | Do | If | Else | While
            | Return => Some("keyword"),
            LBrace | RBrace | LParen | RParen | LBracket | RBracket | Dot | Comma | Semicolon
            | Plus | Minus | Asterisk | Slash | And | LT | GT | EQ | Not => Some("symbol"),
            IntegerConstantKind => Some("integerConstant"),
            StringConstantKind => Some("stringConstant"),
            IdentifierKind => Some("identifier"),
            _ => None,
        }
    }
}

pub struct Tokenizer<'a> {
    source_len: usize,
    chars: Chars<'a>,
    init_pos: u32,
    read_pos: u32,
    source: &'a str,
}

pub trait XML {
    fn xml(&self) -> String;
}

impl<'a> XML for Token<'a> {
    fn xml(&self) -> String {
        match self.kind.as_str() {
            Some(kind_str) => format!("<{0}> {1} </{0}>", kind_str, self.literal),
            None => "".to_string(),
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            source_len: source.len(),
            chars: source.chars(),
            init_pos: 0,
            read_pos: 0,
            source,
        }
    }

    pub fn tokenize(&'a mut self) -> impl Iterator<Item = Token> {
        std::iter::from_fn(move || self.first_token())
    }

    fn nth_char(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or('\0')
    }

    fn first(&self) -> char {
        self.nth_char(0)
    }

    fn second(&self) -> char {
        self.nth_char(1)
    }

    fn first_token(&mut self) -> Option<Token<'a>> {
        if self.source.is_empty() {
            return None;
        }

        let kind = match self.chars.next()? {
            '/' => match self.first() {
                '*' => self.comment(),
                '/' => self.eol_comment(),
                _ => Slash,
            },
            '{' => LBrace,
            '}' => RBrace,
            '(' => LParen,
            ')' => RParen,
            '[' => LBracket,
            ']' => RBracket,
            '.' => Dot,
            ',' => Comma,
            ';' => Semicolon,
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '&' => And,
            '<' => LT,
            '>' => GT,
            '=' => EQ,
            '~' => Not,
            '0'..='9' => self.integer_constant(),
            '"' => self.string_constant(),
            c if is_whitespace(c) => self.whitespace(),
            c if is_identifier_start(c) => self.identifier_or_kw(),
            unknown => panic!("unknown token type of {}", unknown),
        };

        Some(Token {
            kind,
            literal: Literal::String("*"),
        })

        // let token = self.first_token();
        // self.source = &self.source[token.len..];
        // Some(token)
    }

    fn identifier_or_kw(&mut self) -> TokenKind {
        TokenKind::Identifier
    }

    fn integer_constant(&mut self) -> TokenKind {
        TokenKind::IntegerConstant
    }

    fn string_constant(&mut self) -> TokenKind {
        TokenKind::StringConstant
    }

    fn comment(&mut self) -> TokenKind {
        TokenKind::Comment
    }

    fn whitespace(&mut self) -> TokenKind {
        TokenKind::Trivia
    }

    fn eol_comment(&mut self) -> TokenKind {
        TokenKind::Comment
    }
}

fn is_identifier_start(c: char) -> bool {
    false
}

fn is_whitespace(c: char) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::Tokenizer;

    use super::{TokenKind, XML};

    #[test]
    fn test_single() {
        let mut tokenizer = Tokenizer::new("*");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::Asterisk);
        assert_eq!(token.xml(), "<symbol> * </symbol>".to_string());
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        Tokenizer::new("!").tokenize().next();
    }
}
