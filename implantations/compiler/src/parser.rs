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

#[allow(non_snake_case)]
fn P<T>(inner: T) -> Rc<T> {
    Rc::new(inner)
}

use Node::*;

impl<'a> Node<'a> {
    fn as_tag(&self) -> &'static str {
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

    pub fn xml(&self) -> String {
        self.xml_with_indent(0)
    }

    fn xml_with_indent(&self, indent: usize) -> String {
        let mut out = vec![];
        let wrap_tag = self.as_tag();
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
                .for_each(|node| out.push(node.xml_with_indent(indent + incr))),
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

macro_rules! is {
    ($( $pattern:pat )|+) => {
        move |t: &Token<'a>| matches!(t.kind, $( $pattern )|+)
    }
}

#[cfg(test)]
const CYAN: &'static str = "\x1b[96m";
#[cfg(test)]
const END: &'static str = "\x1b[0m";

macro_rules! scope {
    ($( $x:expr )+) => {
        #[cfg(test)]
        println!("enter: \"{}{}{}\"", CYAN, $($x)+, END);
    };
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

    fn var_dec(&mut self) -> Node<'a> {
        scope!("<var-dec>");
        let mut children = vec![
            self.token(is!(Tk::Var)),
            self.token(is_type),
            self.token(is!(Tk::Ident)),
        ];
        while let Some(comma) = self.try_token(is!(Tk::Comma)) {
            children.push(comma);
            children.push(self.token(is!(Tk::Ident)));
        }
        children.push(self.token(is!(Tk::Semi)));
        VarDec(P(children))
    }

    fn try_statement(&mut self) -> Option<Node<'a>> {
        match self.peek_token()?.kind {
            Tk::Let => {
                scope!("<let-statement>");
                let mut children = vec![Syntax(P(self.take_token())), self.token(is!(Tk::Ident))];
                if let Some(bracket) = self.try_token(is!(Tk::LBracket)) {
                    children.push(bracket);
                    children.push(self.expr().expect("expected expr"));
                    children.push(self.token(is!(Tk::RBracket)));
                };
                children.push(self.token(is!(Tk::EQ)));
                children.push(self.expr().expect("expected expr"));
                children.push(self.token(is!(Tk::Semi)));
                Some(LetStatement(P(children)))
            }
            Tk::If => {
                scope!("<if-statement>");
                let mut children = vec![
                    Syntax(P(self.take_token())),
                    self.token(is!(Tk::LParen)),
                    self.expr().expect("expected expr"),
                    self.token(is!(Tk::RParen)),
                    self.token(is!(Tk::LBrace)),
                    self.statements(),
                    self.token(is!(Tk::RBrace)),
                ];
                if let Some(else_node) = self.try_token(is!(Tk::Else)) {
                    children.push(else_node);
                    children.push(self.token(is!(Tk::LBrace)));
                    children.push(self.statements());
                    children.push(self.token(is!(Tk::RBrace)));
                };
                Some(IfStatement(P(children)))
            }
            Tk::While => {
                scope!("<while-statement>");
                Some(WhileStatement(P(vec![
                    self.token(is!(Tk::While)),
                    self.token(is!(Tk::LParen)),
                    self.expr().expect("expected expr"),
                    self.token(is!(Tk::RParen)),
                    self.token(is!(Tk::LBrace)),
                    self.statements(),
                    self.token(is!(Tk::RBrace)),
                ])))
            }
            Tk::Do => {
                scope!("<do-statement>");
                let op = self.token(is!(Tk::Do));
                let name = self.take_token();
                let symbol = self.peek_token()?;
                Some(DoStatement(P(vec![
                    op,
                    self.subroutine_call(name, symbol),
                    self.token(is!(Tk::Semi)),
                ])))
            }
            Tk::Return => {
                scope!("<return-statement>");
                let mut children = vec![self.token(is!(Tk::Return))];
                if let Some(semi) = self.try_token(is!(Tk::Semi)) {
                    children.push(semi);
                } else {
                    children.push(self.expr().expect("expected expr"));
                    children.push(self.token(is!(Tk::Semi)));
                }
                Some(ReturnStatement(P(children)))
            }
            _ => None,
        }
    }

    fn statements(&mut self) -> Node<'a> {
        scope!("<statements>");
        let mut children = vec![];
        while let Some(statement) = self.try_statement() {
            children.push(statement);
        }
        Statements(P(children))
    }

    fn take_token(&mut self) -> Token<'a> {
        self.tokenizer
            .take_token()
            .expect("expect token but there is no more")
    }

    fn peek_token(&mut self) -> Option<Token<'a>> {
        self.tokenizer.peek_token()
    }

    fn token(&mut self, predicate: impl Fn(&Token<'a>) -> bool) -> Node<'a> {
        let token = self.take_token();
        if predicate(&token) {
            return VarName(P(token));
        }
        panic!("{:?} does not fit in predicate function", token)
    }

    fn try_token(&mut self, predicate: impl Fn(&Token<'a>) -> bool) -> Option<Node<'a>> {
        if predicate(&self.peek_token()?) {
            return Some(VarName(P(self.take_token())));
        }
        None
    }

    pub fn class(&mut self) -> Node<'a> {
        scope!("<class>");
        let mut children = vec![
            self.token(is!(Tk::Class)),
            self.token(is!(Tk::Ident)),
            self.token(is!(Tk::LBrace)),
        ];
        while let Some(node) = self.try_class_var_dec() {
            children.push(node);
        }
        while let Some(node) = self.try_subroutine_dec() {
            children.push(node);
        }
        children.push(self.token(is!(Tk::RBrace)));
        Class(P(children))
    }

    fn try_class_var_dec(&mut self) -> Option<Node<'a>> {
        scope!("<class-var-dec>");
        let mut children = vec![
            self.try_token(is!(Tk::Static | Tk::Field))?,
            self.token(is_type),
            self.token(is!(Tk::Ident)),
        ];
        while let Some(comma) = self.try_token(is!(Tk::Comma)) {
            children.push(comma);
            children.push(self.token(is!(Tk::Ident)));
        }
        children.push(self.token(is!(Tk::Semi)));
        Some(ClassVarDec(P(children)))
    }

    fn try_subroutine_dec(&mut self) -> Option<Node<'a>> {
        scope!("<subroutine-dec>");
        let children = vec![
            self.try_token(is!(Tk::Constructor | Tk::Function | Tk::Method))?,
            self.token(|t| is_type(t) || t.kind == Tk::Void),
            self.token(is!(Tk::Ident)),
            self.token(is!(Tk::LParen)),
            self.parameter_list(),
            self.token(is!(Tk::RParen)),
            self.subroutine_body(),
        ];
        Some(SubroutineDec(P(children)))
    }

    fn parameter_list(&mut self) -> Node<'a> {
        scope!("<parameter-list>");
        if let Some(node) = self.try_token(is_type) {
            let mut children = vec![node, self.token(is!(Tk::Ident))];
            while let Some(comma) = self.try_token(is!(Tk::Comma)) {
                children.push(comma);
                children.push(self.token(is_type));
                children.push(self.token(is!(Tk::Ident)));
            }
            return ParameterList(P(children));
        };
        ParameterList(P(vec![]))
    }

    fn subroutine_body(&mut self) -> Node<'a> {
        scope!("<subroutine-body>");
        let mut children = vec![self.token(is!(Tk::LBrace))];
        loop {
            match self.peek_token() {
                Some(t) if t.kind == Tk::Var => {
                    children.push(self.var_dec());
                }
                _ => break,
            }
        }
        children.push(self.statements());
        children.push(self.token(is!(Tk::RBrace)));
        SubroutineBody(P(children))
    }

    fn term(&mut self) -> Option<Node<'a>> {
        scope!("<term>");
        // let ret = Vec::new();
        let inner = match self.peek_token()?.kind {
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
            | Tk::Return => vec![Syntax(P(self.take_token()))],
            Tk::Ident => {
                let token = self.take_token();
                match self.peek_token() {
                    Some(t) if t.kind == Tk::LBracket => vec![
                        VarName(P(token)),
                        Syntax(P(self.take_token())),
                        self.expr().expect("expected expr"),
                        self.token(is!(Tk::RBracket)),
                    ],
                    Some(t) if is!(Tk::LParen | Tk::Dot)(&t) => {
                        vec![self.subroutine_call(token, t)]
                    }
                    _ => vec![VarName(P(token))],
                }
            }
            Tk::LParen => vec![
                Syntax(P(self.take_token())),
                self.expr().expect("expected expr"),
                self.token(is!(Tk::RParen)),
            ],
            t if matches!(t, Tk::Minus | Tk::Not) => {
                vec![UnaryOp(P(self.take_token())), self.term().unwrap()]
            }
            _ => vec![],
        };
        if inner.is_empty() {
            None
        } else {
            Some(Term(P(inner)))
        }
    }

    fn subroutine_call(&mut self, name: Token<'a>, symbol: Token<'a>) -> Node<'a> {
        scope!("<subroutine-call>");
        match symbol.kind {
            Tk::LParen => SubroutineCall(P(vec![
                SubroutineName(P(name)),
                Syntax(P(self.take_token())),
                self.expr_list(),
                self.token(is!(Tk::RParen)),
            ])),
            Tk::Dot => SubroutineCall(P(vec![
                ClassName(P(name)),
                Syntax(P(self.take_token())),
                self.token(is!(Tk::Ident)),
                self.token(is!(Tk::LParen)),
                self.expr_list(),
                self.token(is!(Tk::RParen)),
            ])),
            _ => panic!("symbol {:?} is not expected in subroutine call", symbol),
        }
    }

    fn expr(&mut self) -> Option<Node<'a>> {
        scope!("<expr>");
        let mut children = vec![];
        children.push(self.term()?);
        while let Some(op) = self.try_token(is!(Tk::Plus
            | Tk::Minus
            | Tk::Asterisk
            | Tk::Slash
            | Tk::And
            | Tk::Or
            | Tk::LT
            | Tk::GT
            | Tk::EQ))
        {
            children.push(op);
            children.push(self.term().expect("expect term but not found"));
        }
        Some(Expression(P(children)))
    }

    fn expr_list(&mut self) -> Node<'a> {
        scope!("<expr-list>");
        let mut children = vec![];
        if let Some(expr) = self.expr() {
            children.push(expr);
        }
        while let Some(comma) = self.try_token(is!(Tk::Comma)) {
            children.push(comma);
            children.push(self.expr().expect("expected expr after comma"));
        }
        ExpressionList(P(children))
    }
}

fn is_type<'a>(token: &Token<'a>) -> bool {
    matches!(token.kind, Tk::Int | Tk::Char | Tk::Boolean | Tk::Ident)
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
        let tree = Parser::new(&jack).class().xml();

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
            node.xml()
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
