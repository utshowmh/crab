use crate::common::types::Object;

use super::{
    syntax_tree::{
        BinaryExpression, Expression, LiteralExpression, ParenthesizedExpression, UnaryExpression,
    },
    token::{Token, TokenKind},
};

pub(super) struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub(super) diagnostics: Vec<String>,
}

impl Parser {
    pub(super) fn new(tokens: Vec<Token>, diagnostics: Vec<String>) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostics,
        }
    }

    pub(super) fn parse(&mut self) -> Expression {
        let expression = self.parse_expression();
        self.match_token(TokenKind::Eof);
        expression
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_or_expression()
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
                Expression::Literal(LiteralExpression::new(token, Object::Boolean(value)))
            }
            _ => {
                let token = self.match_token(TokenKind::Number);
                let value = token.lexeme.parse().unwrap();
                Expression::Literal(LiteralExpression::new(token, Object::Number(value)))
            }
        }
    }

    fn peek(&self, offset: usize) -> Token {
        let index = offset + self.current;
        if index < self.tokens.len() {
            self.tokens[index].clone()
        } else {
            Token::new(TokenKind::Eof, "\0".to_string())
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
        let peek_kind = self.peek(0).kind;
        if peek_kind == kind {
            self.next_token()
        } else {
            self.diagnostics
                .push(format!("Unexpected token '{peek_kind}', expected '{kind}'"));
            self.advance();
            Token::new(kind, "0".to_string())
        }
    }

    fn token_matches(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.peek(0).kind)
    }
}
