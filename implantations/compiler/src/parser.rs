use crate::{Token, TokenKind, Tokenizer};

pub enum Node<'a> {
    Class(Token<'a>),
    ClassVarDec(Vec<Node<'a>>),
    Type(Token<'a>),
    SubroutineDec(Vec<Node<'a>>),
    ParameterList(Vec<Node<'a>>),
    SubroutineBody(Vec<Node<'a>>),
    VarDec(Vec<Node<'a>>),
    ClassName(Token<'a>),
    SubroutineName(Token<'a>),
    VarName(Token<'a>),
    // Statements
    Statement(Vec<Node<'a>>),
    LetStatement(Vec<Node<'a>>),
    IfStatement(Vec<Node<'a>>),
    WhileStatement(Vec<Node<'a>>),
    DoStatement(Vec<Node<'a>>),
    ReturnStatement(Vec<Node<'a>>),
    // Expressions
    Term(Vec<Node<'a>>),
    SubroutineCall(Vec<Node<'a>>),
    ExpressionList(Vec<Node<'a>>),
    Op(Token<'a>),
    UnaryOp(Token<'a>),
    KeywordConstant(Token<'a>),
    Syntax(Token<'a>),
}

use Node::*;

impl<'a> Node<'a> {
    fn as_str(&self) -> &'static str {
        match self {
            Class(..) => "class",
            ClassVarDec(..) => "classVarDec",

            SubroutineDec(..) => "subroutineDec",
            ParameterList(..) => "parameterList",
            SubroutineBody(..) => "subroutineBody",
            VarDec(..) => "varDec",

            // Statements
            LetStatement(..) => "letStatement",
            IfStatement(..) => "ifStatement",
            WhileStatement(..) => "whileStatement",
            DoStatement(..) => "doStatement",
            ReturnStatement(..) => "returnStatement",
            // Expressions
            Term(..) => "term",

            ExpressionList(..) => "expressionList",
            Syntax(..) | Type(..) | ClassName(..) | SubroutineName(..) | VarName(..)
            | Statement(..) | SubroutineCall(..) | Op(..) | UnaryOp(..) | KeywordConstant(..) => "",
        }
    }

    pub fn xml(&self, indent: usize) -> String {
        let mut out = Vec::new();
        let wrap_tag = self.as_str();
        let pad = " ".repeat(indent);

        if !wrap_tag.is_empty() {
            out.push(format!("{}<{}>", pad, wrap_tag));
        }
        match self {
            ClassVarDec(children)
            | SubroutineDec(children)
            | ParameterList(children)
            | SubroutineBody(children)
            | VarDec(children)
            | Statement(children)
            | LetStatement(children)
            | IfStatement(children)
            | WhileStatement(children)
            | DoStatement(children)
            | ReturnStatement(children)
            | Term(children)
            | SubroutineCall(children)
            | ExpressionList(children) => children
                .iter()
                .for_each(|node| out.push(node.xml(indent + 1))),
            Class(token)
            | Syntax(token)
            | Type(token)
            | ClassName(token)
            | SubroutineName(token)
            | VarName(token)
            | Op(token)
            | UnaryOp(token)
            | KeywordConstant(token) => {
                out.push(format!("{}{}", pad, token.xml()));
            }
        };
        if !wrap_tag.is_empty() {
            out.push(format!("{}</{}>", pad, wrap_tag));
        }
        out.join("\n")
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let p = Self {
            tokenizer: Tokenizer::new(source),
        };
        p
    }

    pub fn parse(&mut self) -> Node<'a> {
        let token = self.tokenizer.first_token_non_trivia().unwrap();
        match token.kind {
            TokenKind::Var => Node::VarDec(vec![
                Node::Syntax(token),
                self.parse_type(),
                self.parse_identifier(),
                self.parse_symbol(TokenKind::Semicolon),
            ]),
            _ => panic!(format!("unexpected token {:?}", token)),
        }
    }

    fn parse_symbol(&mut self, kind: TokenKind) -> Node<'a> {
        let token = self
            .tokenizer
            .first_token_non_trivia()
            .expect("expect symbol token but there is no more");
        match &token.kind {
            k if k == &kind => Node::Syntax(token),
            _ => panic!(),
        }
    }

    fn parse_type(&mut self) -> Node<'a> {
        let token = self
            .tokenizer
            .first_token_non_trivia()
            .expect("expect type token but there is no more");
        match token.kind {
            TokenKind::Int | TokenKind::Char | TokenKind::Boolean | TokenKind::Identifier => {
                Node::Type(token)
            }
            _ => panic!(format!("unexpected token kind {:?}", token.kind)),
        }
    }

    fn parse_identifier(&mut self) -> Node<'a> {
        let token = self
            .tokenizer
            .first_token_non_trivia()
            .expect("expect identifier token but there is no more");
        match token.kind {
            TokenKind::Identifier => Node::VarName(token),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_enum_size() {
    //     println!("sizeof Node enum: {}", std::mem::size_of::<Node>());
    // }

    #[test]
    fn test_var_dec() {
        let node = Parser::new("var int x;").parse();
        assert_eq!(
            r#"<varDec>
 <keyword> var </keyword>
 <keyword> int </keyword>
 <identifier> x </identifier>
 <symbol> ; </symbol>
</varDec>"#,
            node.xml(0)
        )
    }
}
