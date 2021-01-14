use crate::{Token, TokenKind, Tokenizer};
use std::rc::Rc;
pub enum Node<'a> {
    Class(Rc<Token<'a>>),
    ClassVarDec(Rc<Vec<Node<'a>>>),
    Type(Rc<Token<'a>>),
    SubroutineDec(Rc<Vec<Node<'a>>>),
    ParameterList(Rc<Vec<Node<'a>>>),
    SubroutineBody(Rc<Vec<Node<'a>>>),
    VarDec(Rc<Vec<Node<'a>>>),
    ClassName(Rc<Token<'a>>),
    SubroutineName(Rc<Token<'a>>),
    VarName(Rc<Token<'a>>),
    // Statements
    Statement(Rc<Vec<Node<'a>>>),
    LetStatement(Rc<Vec<Node<'a>>>),
    IfStatement(Rc<Vec<Node<'a>>>),
    WhileStatement(Rc<Vec<Node<'a>>>),
    DoStatement(Rc<Vec<Node<'a>>>),
    ReturnStatement(Rc<Vec<Node<'a>>>),
    // Expressions
    Term(Rc<Vec<Node<'a>>>),
    SubroutineCall(Rc<Vec<Node<'a>>>),
    ExpressionList(Rc<Vec<Node<'a>>>),
    Op(Rc<Token<'a>>),
    UnaryOp(Rc<Token<'a>>),
    KeywordConstant(Rc<Token<'a>>),
    Syntax(Rc<Token<'a>>),
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
            LetStatement(..) => "letStatement",
            IfStatement(..) => "ifStatement",
            WhileStatement(..) => "whileStatement",
            DoStatement(..) => "doStatement",
            ReturnStatement(..) => "returnStatement",
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
            TokenKind::Var => Node::VarDec(Rc::new(vec![
                Node::Syntax(Rc::new(token)),
                self.parse_type(),
                self.parse_identifier(),
                self.parse_symbol(TokenKind::Semicolon),
            ])),
            _ => panic!(format!("unexpected token {:?}", token)),
        }
    }

    fn parse_symbol(&mut self, kind: TokenKind) -> Node<'a> {
        let token = self
            .tokenizer
            .first_token_non_trivia()
            .expect("expect symbol token but there is no more");
        match &token.kind {
            k if k == &kind => Node::Syntax(Rc::new(token)),
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
                Node::Type(Rc::new(token))
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
            TokenKind::Identifier => Node::VarName(Rc::new(token)),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_size() {
        assert_eq!(std::mem::size_of::<Rc<Node>>(), 8);
    }

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
