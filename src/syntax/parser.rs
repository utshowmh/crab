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
        let expression = self.parse_expression(0);
        self.match_token(TokenKind::Eof);
        expression
    }

    fn parse_expression(&mut self, parent_precedence: usize) -> Expression {
        let mut left: Expression;
        let unary_precedence = self.peek(0).kind.get_unary_precedence();
        if unary_precedence != 0 && unary_precedence >= parent_precedence {
            let operator = self.next_token();
            let right = self.parse_expression(unary_precedence);
            left = Expression::Unary(UnaryExpression::new(operator, right));
        } else {
            left = self.parse_primary_expression();
        }

        loop {
            let binary_precedence = self.peek(0).kind.get_binary_precedence();
            if binary_precedence == 0 && binary_precedence <= parent_precedence {
                break;
            }
            let operator = self.next_token();
            let right = self.parse_expression(binary_precedence);
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        left
    }

    fn parse_primary_expression(&mut self) -> Expression {
        match self.peek(0).kind {
            TokenKind::OpenParen => {
                self.next_token();
                let expression = self.parse_expression(0);
                self.match_token(TokenKind::CloseParen);
                Expression::Parenthesized(ParenthesizedExpression::new(expression))
            }
            _ => {
                let token = self.match_token(TokenKind::Number);
                let value = token.lexeme.parse().unwrap_or(0);
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
}
