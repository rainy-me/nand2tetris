use std::unimplemented;

use crate::{tokenizer, Token, TokenKind, Tokenizer};

pub enum NodeKind {
    Root,
    Syntax,
    Class,
    ClassVarDec,
    Type,
    SubroutineDec,
    ParameterList,
    SubroutineBody,
    VarDec,
    ClassName,
    SubroutineName,
    VarName,
    // Statements
    Statement,
    LetStatement,
    IfStatement,
    WhileStatement,
    DoStatement,
    ReturnStatement,
    // Expressions
    Term,
    SubroutineCall,
    ExpressionList,
    Op,
    UnaryOp,
    KeywordConstant,
}

use NodeKind::*;

impl NodeKind {
    fn as_str(&self) -> &'static str {
        match self {
            Class => "class",
            ClassVarDec => "classVarDec",

            SubroutineDec => "subroutineDec",
            ParameterList => "parameterList",
            SubroutineBody => "subroutineBody",
            VarDec => "varDec",

            // Statements
            LetStatement => "letStatement",
            IfStatement => "ifStatement",
            WhileStatement => "whileStatement",
            DoStatement => "doStatement",
            ReturnStatement => "returnStatement",
            // Expressions
            Term => "term",

            ExpressionList => "expressionList",
            Root | Syntax | Type | ClassName | SubroutineName | VarName | Statement
            | SubroutineCall | Op | UnaryOp | KeywordConstant => "",
        }
    }
}

struct Node<'a> {
    kind: NodeKind,
    tokens: Option<Vec<Token<'a>>>,
    children: Option<Vec<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn wrap(kind: NodeKind, children: Option<Vec<Node<'a>>>) -> Self {
        Self {
            kind,
            tokens: None,
            children,
        }
    }
    fn from(kind: NodeKind, token: Token<'a>) -> Self {
        Self {
            kind,
            tokens: Some(vec![token]),
            children: None,
        }
    }
    pub fn xml(&self, indent: usize) -> String {
        let mut out = Vec::new();
        let wrap_tag = self.kind.as_str();
        let pad = " ".repeat(indent);

        if !wrap_tag.is_empty() {
            out.push(format!("{}<{}>", pad, wrap_tag));
        }
        self.children
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .for_each(|node| out.push(node.xml(indent + 1)));
        self.tokens
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .for_each(|token| out.push(format!("{}{}", pad, token.xml())));
        if !wrap_tag.is_empty() {
            out.push(format!("{}</{}>", pad, wrap_tag));
        }
        out.join("\n")
    }
}

struct Parser<'a> {
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
            TokenKind::Var => {
                let children = vec![
                    Node::from(NodeKind::Syntax, token),
                    self.parse_type(),
                    self.parse_identifier(),
                    self.parse_symbol(TokenKind::Semicolon),
                ];
                Node::wrap(NodeKind::VarDec, Some(children))
            }
            _ => panic!(format!("unexpected token {:?}", token)),
        }
    }

    fn parse_symbol(&mut self, kind: TokenKind) -> Node<'a> {
        let token = self
            .tokenizer
            .first_token_non_trivia()
            .expect("expect symbol token but there is no more");
        match &token.kind {
            k if k == &kind => Node::from(NodeKind::Type, token),
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
                Node::from(NodeKind::Type, token)
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
            TokenKind::Identifier => Node::from(NodeKind::VarName, token),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
