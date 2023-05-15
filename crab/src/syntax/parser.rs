use std::{cell::RefCell, rc::Rc};

use crate::common::{
    diagnostic::{DiagnosticBag, Position},
    types::Object,
};

use super::{
    syntax_tree::{
        AssignmentExpression, BinaryExpression, Expression, ExpressionStatement, LiteralExpression,
        NameExpression, ParenthesizedExpression, PrintStatement, Statement, UnaryExpression,
    },
    token::{Token, TokenKind},
};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub(super) diagnostic_bag: Rc<RefCell<DiagnosticBag>>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>, diagnostic_bag: Rc<RefCell<DiagnosticBag>>) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostic_bag,
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        while self.peek(0).kind != TokenKind::Eof {
            statements.push(self.parse_statement());
        }
        self.match_token(TokenKind::Eof);
        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek(0).kind {
            TokenKind::Print => {
                self.advance();
                Statement::Print(PrintStatement::new(self.parse_expression()))
            }
            _ => Statement::Expression(ExpressionStatement::new(self.parse_expression())),
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Expression {
        if self.peek(0).kind == TokenKind::Identifier && self.peek(1).kind == TokenKind::Equal {
            let identifier = self.next_token();
            let equal = self.next_token();
            let expression = self.parse_assignment_expression();
            Expression::Assignment(AssignmentExpression::new(identifier, equal, expression))
        } else {
            self.parse_or_expression()
        }
    }

    fn parse_or_expression(&mut self) -> Expression {
        let mut left = self.parse_and_expression();
        while self.token_matches(&[TokenKind::PipePipe]) {
            let operator = self.next_token();
            let right = self.parse_and_expression();
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }
        left
    }

    fn parse_and_expression(&mut self) -> Expression {
        let mut left = self.parse_equality_expression();
        while self.token_matches(&[TokenKind::AmpersandAmpersand]) {
            let operator = self.next_token();
            let right = self.parse_equality_expression();
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }
        left
    }

    fn parse_equality_expression(&mut self) -> Expression {
        let mut left = self.parse_additive_expression();
        while self.token_matches(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.next_token();
            let right = self.parse_additive_expression();
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }
        left
    }

    fn parse_additive_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicative_expression();
        while self.token_matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.next_token();
            let right = self.parse_multiplicative_expression();
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }
        left
    }

    fn parse_multiplicative_expression(&mut self) -> Expression {
        let mut left = self.parse_unary_expression();
        while self.token_matches(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.next_token();
            let right = self.parse_unary_expression();
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }
        left
    }

    fn parse_unary_expression(&mut self) -> Expression {
        if self.token_matches(&[TokenKind::Plus, TokenKind::Minus, TokenKind::Bang]) {
            let operator = self.next_token();
            let right = self.parse_unary_expression();
            Expression::Unary(UnaryExpression::new(operator, right))
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_primary_expression(&mut self) -> Expression {
        match self.peek(0).kind {
            TokenKind::OpenParen => {
                self.next_token();
                let expression = self.parse_expression();
                self.match_token(TokenKind::CloseParen);
                Expression::Parenthesized(ParenthesizedExpression::new(expression))
            }
            TokenKind::True | TokenKind::False => {
                let token = self.next_token();
                let value = token.lexeme.parse().unwrap();
                Expression::Literal(LiteralExpression::new(Object::Boolean(value)))
            }
            TokenKind::Number => {
                let token = self.next_token();
                let value = token.lexeme.parse().unwrap();
                Expression::Literal(LiteralExpression::new(Object::Number(value)))
            }
            _ => {
                let identifier = self.match_token(TokenKind::Identifier);
                Expression::Name(NameExpression::new(identifier))
            }
        }
    }

    fn peek(&self, offset: usize) -> Token {
        let index = offset + self.current;
        if index < self.tokens.len() {
            self.tokens[index].clone()
        } else {
            Token::new(TokenKind::Eof, "EOF".to_string(), Position::new(0, 0))
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = self.peek(0);
        self.advance();
        token
    }

    fn match_token(&mut self, kind: TokenKind) -> Token {
        let token = self.peek(0);
        if kind == token.kind {
            self.next_token()
        } else {
            self.diagnostic_bag.borrow_mut().unexpected_token(
                token.position,
                kind.clone(),
                token.kind,
            );
            let token = Token::new(kind, "GENERATED".to_string(), self.peek(0).position);
            self.advance();
            token
        }
    }

    fn token_matches(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.peek(0).kind)
    }
}
