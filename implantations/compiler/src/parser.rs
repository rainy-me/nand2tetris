use crate::{Token, TokenKind as Tk, Tokenizer};
use std::rc::Rc;

pub enum Node<'a> {
    Class(Rc<Vec<Node<'a>>>),
    ClassVarDec(Rc<Vec<Node<'a>>>),
    Type(Rc<Token<'a>>),
    SubroutineDec(Rc<Vec<Node<'a>>>),
    ParameterList(Rc<Vec<Node<'a>>>),
    SubroutineBody(Rc<Vec<Node<'a>>>),
    VarDec(Rc<Vec<Node<'a>>>),
    ClassName(Rc<Token<'a>>),
    SubroutineName(Rc<Token<'a>>),
    VarName(Rc<Token<'a>>),
    // Statement
    Statement(Rc<Vec<Node<'a>>>),
    Statements(Rc<Vec<Node<'a>>>),
    LetStatement(Rc<Vec<Node<'a>>>),
    IfStatement(Rc<Vec<Node<'a>>>),
    WhileStatement(Rc<Vec<Node<'a>>>),
    DoStatement(Rc<Vec<Node<'a>>>),
    ReturnStatement(Rc<Vec<Node<'a>>>),
    // Expressions
    Expression(Rc<Vec<Node<'a>>>),
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
            Expression(..) => "expression",
            ExpressionList(..) => "expressionList",
            Statements(..) => "statements",
            Syntax(..) | Type(..) | ClassName(..) | SubroutineName(..) | VarName(..)
            | Statement(..) | SubroutineCall(..) | Op(..) | UnaryOp(..) | KeywordConstant(..) => "",
        }
    }

    pub fn xml(&self, indent: usize) -> String {
        let mut out = Vec::new();
        let wrap_tag = self.as_str();
        let pad = " ".repeat(indent * 2);
        let incr = if wrap_tag.is_empty() { 0 } else { 1 };

        if !wrap_tag.is_empty() {
            out.push(format!("{}<{}>", pad, wrap_tag));
        }
        match self {
            Class(children)
            | ClassVarDec(children)
            | SubroutineDec(children)
            | ParameterList(children)
            | SubroutineBody(children)
            | VarDec(children)
            | Statement(children)
            | Statements(children)
            | LetStatement(children)
            | IfStatement(children)
            | WhileStatement(children)
            | DoStatement(children)
            | ReturnStatement(children)
            | Term(children)
            | SubroutineCall(children)
            | Expression(children)
            | ExpressionList(children) => children
                .iter()
                .for_each(|node| out.push(node.xml(indent + incr))),
            Syntax(token)
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
        Self {
            tokenizer: Tokenizer::new(source),
        }
    }

    pub fn parse(&mut self) -> Node<'a> {
        // jack source starts with a class.
        self.class()
    }

    pub fn var_dec(&mut self) -> Node<'a> {
        debug("<var-dec>");
        let mut children = vec![
            self.token_exact(Tk::Var),
            self.token(is_type),
            self.token(is_identifier),
        ];

        while let Some(comma) = self.try_token_exact(Tk::Comma) {
            children.push(comma);
            children.push(self.token(is_identifier));
        }
        children.push(self.token_exact(Tk::Semicolon));
        Node::VarDec(Rc::new(children))
    }

    fn try_statement(&mut self) -> Option<Node<'a>> {
        let next = self.peek_next_token()?;
        match next.kind {
            Tk::Let => {
                debug("<let-statement>");
                let mut children = vec![
                    Node::Syntax(Rc::new(self.expect_take_token())),
                    self.token(is_identifier),
                ];
                if let Some(bracket) = self.try_token_exact(Tk::LBracket) {
                    children.push(bracket);
                    children.push(self.expr().expect("expected expr"));
                    children.push(self.token_exact(Tk::RBracket));
                };
                children.push(self.token_exact(Tk::EQ));
                children.push(self.expr().expect("expected expr"));
                children.push(self.token_exact(Tk::Semicolon));
                Some(Node::LetStatement(Rc::new(children)))
            }
            Tk::If => {
                debug("<if-statement>");
                let mut children = vec![
                    Node::Syntax(Rc::new(self.expect_take_token())),
                    self.token_exact(Tk::LParen),
                    self.expr().expect("expected expr"),
                    self.token_exact(Tk::RParen),
                    self.token_exact(Tk::LBrace),
                    self.statements(),
                    self.token_exact(Tk::RBrace),
                ];
                if let Some(else_node) = self.try_token_exact(Tk::Else) {
                    children.push(else_node);
                    children.push(self.token_exact(Tk::LBrace));
                    children.push(self.statements());
                    children.push(self.token_exact(Tk::RBrace));
                };
                Some(Node::IfStatement(Rc::new(children)))
            }
            Tk::While => {
                debug("<while-statement>");
                let children = vec![
                    self.token_exact(Tk::While),
                    self.token_exact(Tk::LParen),
                    self.expr().expect("expected expr"),
                    self.token_exact(Tk::RParen),
                    self.token_exact(Tk::LBrace),
                    self.statements(),
                    self.token_exact(Tk::RBrace),
                ];
                Some(Node::WhileStatement(Rc::new(children)))
            }
            Tk::Do => {
                debug("<do-statement>");
                let op = self.token_exact(Tk::Do);
                let name = self.expect_take_token();
                let symbol = self.peek_next_token()?;
                let children = vec![
                    op,
                    self.subroutine_call(name, symbol),
                    self.token_exact(Tk::Semicolon),
                ];
                Some(Node::DoStatement(Rc::new(children)))
            }
            Tk::Return => {
                debug("<return-statement>");
                let mut children = vec![self.token_exact(Tk::Return)];
                if let Some(semi) = self.try_token_exact(Tk::Semicolon) {
                    children.push(semi);
                } else {
                    children.push(self.expr().expect("expected expr"));
                    children.push(self.token_exact(Tk::Semicolon));
                }
                Some(Node::ReturnStatement(Rc::new(children)))
            }
            _ => None,
        }
    }

    fn statements(&mut self) -> Node<'a> {
        debug("<statements>");
        let mut children = vec![];
        if let Some(statement) = self.try_statement() {
            children.push(statement);
        }
        while let Some(statement) = self.try_statement() {
            children.push(statement);
        }
        Node::Statements(Rc::new(children))
    }

    fn expect_take_token(&mut self) -> Token<'a> {
        let token = self
            .tokenizer
            .take_token()
            .expect("expect identifier token but there is no more");
        token
    }

    fn peek_next_token(&mut self) -> Option<Token<'a>> {
        self.tokenizer.peek_token()
    }

    pub fn class(&mut self) -> Node<'a> {
        debug("<class>");
        let mut children = vec![
            self.token(|t| t.kind == Tk::Class),
            self.token(is_identifier),
            self.token(|t| t.kind == Tk::LBrace),
        ];

        while let Some(node) = self.try_class_var_dec() {
            children.push(node);
        }
        while let Some(node) = self.try_subroutine_dec() {
            children.push(node);
        }
        children.push(self.token(|t| t.kind == Tk::RBrace));
        Node::Class(Rc::new(children))
    }

    fn try_class_var_dec(&mut self) -> Option<Node<'a>> {
        let token = self.try_token(|t| matches!(t.kind, Tk::Static | Tk::Field))?;
        debug("<class-var-dec>");
        let mut children = vec![token, self.token(is_type), self.token(is_identifier)];
        while let Some(comma) = self.try_token_exact(Tk::Comma) {
            children.push(comma);
            children.push(self.token(is_identifier));
        }
        children.push(self.token_exact(Tk::Semicolon));
        Some(Node::ClassVarDec(Rc::new(children)))
    }

    fn try_subroutine_dec(&mut self) -> Option<Node<'a>> {
        let token =
            self.try_token(|t| matches!(t.kind, Tk::Constructor | Tk::Function | Tk::Method))?;
        debug("<subroutine-dec>");

        let children = vec![
            token,
            self.token(|t| is_type(t) || t.kind == Tk::Void),
            self.token(is_identifier),
            self.token_exact(Tk::LParen),
            self.parameter_list(),
            self.token_exact(Tk::RParen),
            self.subroutine_body(),
        ];
        Some(Node::SubroutineDec(Rc::new(children)))
    }

    fn parameter_list(&mut self) -> Node<'a> {
        debug("<parameter-list>");
        match self.try_token(is_type) {
            Some(node) => {
                let mut children = vec![node, self.token(is_identifier)];
                while let Some(comma) = self.try_token_exact(Tk::Comma) {
                    children.push(comma);
                    children.push(self.token(is_type));
                    children.push(self.token(is_identifier));
                }
                Node::ParameterList(Rc::new(children))
            }
            _ => Node::ParameterList(Rc::new(vec![])),
        }
    }

    fn subroutine_body(&mut self) -> Node<'a> {
        debug("<subroutine-body>");
        let mut children = vec![self.token_exact(Tk::LBrace)];

        loop {
            match self.peek_next_token() {
                Some(t) if t.kind == Tk::Var => {
                    children.push(self.var_dec());
                }
                _ => break,
            }
        }
        children.push(self.statements());
        children.push(self.token_exact(Tk::RBrace));
        Node::SubroutineBody(Rc::new(children))
    }

    fn term(&mut self) -> Option<Node<'a>> {
        debug("<term>");
        // let ret = Vec::new();
        let token = self.peek_next_token()?;
        match token.kind {
            Tk::IntegerConstant
            | Tk::StringConstant
            | Tk::Class
            | Tk::Constructor
            | Tk::Function
            | Tk::Method
            | Tk::Field
            | Tk::Static
            | Tk::Var
            | Tk::Int
            | Tk::Char
            | Tk::Boolean
            | Tk::Void
            | Tk::True
            | Tk::False
            | Tk::Null
            | Tk::This
            | Tk::Let
            | Tk::Do
            | Tk::If
            | Tk::Else
            | Tk::While
            | Tk::Return => Some(Node::Term(Rc::new(vec![Node::Syntax(Rc::new(
                self.expect_take_token(),
            ))]))),
            Tk::Identifier => {
                let token = self.expect_take_token();
                match self.peek_next_token() {
                    Some(t) if t.kind == Tk::LBracket => Some(Node::Term(Rc::new(vec![
                        Node::VarName(Rc::new(token)),
                        Node::Syntax(Rc::new(self.expect_take_token())),
                        self.expr().expect("expected expr"),
                        self.token_exact(Tk::RBracket),
                    ]))),
                    Some(t) if matches!(t.kind, Tk::LParen | Tk::Dot) => {
                        Some(Node::Term(Rc::new(vec![self.subroutine_call(token, t)])))
                    }
                    _ => Some(Node::Term(Rc::new(vec![Node::VarName(Rc::new(token))]))),
                }
            }
            Tk::LParen => Some(Node::Term(Rc::new(vec![
                Node::Syntax(Rc::new(self.expect_take_token())),
                self.expr().expect("expected expr"),
                self.token_exact(Tk::RParen),
            ]))),
            _ if is_unary_op(&token) => Some(Node::Term(Rc::new(vec![
                Node::UnaryOp(Rc::new(self.expect_take_token())),
                self.term().unwrap(),
            ]))),
            _ => None,
        }
    }

    fn subroutine_call(&mut self, name: Token<'a>, symbol: Token<'a>) -> Node<'a> {
        debug("<subroutine-call>");
        match symbol.kind {
            Tk::LParen => Node::SubroutineCall(Rc::new(vec![
                Node::SubroutineName(Rc::new(name)),
                Node::Syntax(Rc::new(self.expect_take_token())),
                self.expr_list(),
                self.token_exact(Tk::RParen),
            ])),
            Tk::Dot => Node::SubroutineCall(Rc::new(vec![
                Node::ClassName(Rc::new(name)),
                Node::Syntax(Rc::new(self.expect_take_token())),
                self.token(is_identifier),
                self.token_exact(Tk::LParen),
                self.expr_list(),
                self.token_exact(Tk::RParen),
            ])),
            _ => panic!(
                "symbol {:?} should not be expected in subroutine call",
                symbol
            ),
        }
    }

    fn expr(&mut self) -> Option<Node<'a>> {
        debug("<expr>");
        let mut children = vec![];
        let term = self.term()?;
        children.push(term);
        while let Some(op) = self.try_token(is_op) {
            children.push(op);
            children.push(self.term().expect("expect term"));
        }
        Some(Node::Expression(Rc::new(children)))
    }

    fn expr_list(&mut self) -> Node<'a> {
        debug("<expr-list>");
        let mut children = vec![];
        if let Some(expr) = self.expr() {
            children.push(expr);
        }
        while let Some(comma) = self.try_token_exact(Tk::Comma) {
            children.push(comma);
            children.push(self.expr().expect("expected expr after comma"));
        }
        Node::ExpressionList(Rc::new(children))
    }

    fn token_exact(&mut self, predicate: Tk) -> Node<'a> {
        let token = self.expect_take_token();
        if token.kind == predicate {
            return Node::VarName(Rc::new(token));
        }
        panic!("{:?} does not fit in predicate {:?}", token, predicate)
    }

    fn try_token_exact(&mut self, predicate: Tk) -> Option<Node<'a>> {
        let token = self.peek_next_token()?;
        if token.kind == predicate {
            return Some(Node::VarName(Rc::new(self.expect_take_token())));
        }
        // panic!("{:?} does not fit in predicate {:?}", token, predicate)
        None
    }

    fn token(&mut self, predicate: impl Fn(&Token<'a>) -> bool) -> Node<'a> {
        let token = self.expect_take_token();
        if predicate(&token) {
            return Node::VarName(Rc::new(token));
        }
        panic!("{:?} does not fit in predicate function", token)
    }

    fn try_token(&mut self, predicate: impl Fn(&Token<'a>) -> bool) -> Option<Node<'a>> {
        let token = self.peek_next_token()?;
        if predicate(&token) {
            return Some(Node::VarName(Rc::new(self.expect_take_token())));
        }
        None
    }
}

fn is_type<'a>(token: &Token<'a>) -> bool {
    matches!(
        token.kind,
        Tk::Int | Tk::Char | Tk::Boolean | Tk::Identifier
    )
}

fn is_op<'a>(token: &Token<'a>) -> bool {
    matches!(
        token.kind,
        Tk::Plus
            | Tk::Minus
            | Tk::Asterisk
            | Tk::Slash
            | Tk::And
            | Tk::Or
            | Tk::LT
            | Tk::GT
            | Tk::EQ
    )
}

fn is_unary_op<'a>(token: &Token<'a>) -> bool {
    matches!(token.kind, Tk::Minus | Tk::Not)
}

fn is_identifier<'a>(token: &Token<'a>) -> bool {
    matches!(token.kind, Tk::Identifier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::path::PathBuf;

    #[cfg(test)]
    fn compare(name: &str) {
        let mut jack_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(OsStr::new("../../projects/10/"));
        let mut xml_path = jack_path.clone();
        let mut out_path = jack_path.clone();
        jack_path.push(name);
        jack_path.set_extension("jack");
        xml_path.push(format!("{}", name));
        xml_path.set_extension("xml");
        out_path.push(format!("{}-out", name));
        out_path.set_extension("xml");

        let jack = std::fs::read_to_string(jack_path)
            .expect("failed to read jack file")
            .to_string();
        let xml_content = std::fs::read_to_string(xml_path)
            .expect("failed to read test file")
            .to_string();
        let tree = Parser::new(&jack).class().xml(0);

        std::fs::write(out_path, &tree).expect("failed to write file");
        // compare line by line to ignore the different newline character
        let mut tokens = tree.lines();
        for xml_line in xml_content.lines() {
            if xml_line.is_empty() {
                continue;
            }
            let token = tokens.next().unwrap();
            assert_eq!(xml_line, token);
        }
    }

    #[test]
    fn test_enum_size() {
        assert_eq!(std::mem::size_of::<Rc<Node>>(), 8);
    }

    #[test]
    fn test_var_dec() {
        let node = Parser::new("var int x;").var_dec();
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

    #[test]
    fn test_array_test() {
        compare("ArrayTest/Main");
    }
    #[test]
    fn test_expression_less_square_main() {
        compare("ExpressionLessSquare/Main");
        compare("ExpressionLessSquare/SquareGame");
        compare("ExpressionLessSquare/Square");
    }

    #[test]
    fn test_square_main() {
        compare("Square/Main");
        compare("Square/SquareGame");
        compare("Square/Square");
    }
}

const CYAN: &'static str = "\x1b[96m";
// const UNDERLINE: &'static str = "\033[4m";
const END: &'static str = "\x1b[0m";
//
fn debug(msg: &str) {
    println!("{}{}{}", CYAN, msg, END);
}
//
