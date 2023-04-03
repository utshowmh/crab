use super::token::{Token, TokenKind};

pub(super) struct Lexer {
    source: Vec<char>,
    current: usize,
    pub(super) diagnostics: Vec<String>,
}

impl Lexer {
    pub(super) fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            diagnostics: Vec::new(),
        }
    }

    pub(super) fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token.kind == TokenKind::Eof {
                tokens.push(token);
                break;
            } else if token.kind == TokenKind::Whitespace || token.kind == TokenKind::Invalid {
                continue;
            } else {
                tokens.push(token);
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        if self.source.len() <= self.current {
            Token::new(TokenKind::Eof, "\0".to_string())
        } else {
            match self.next_char() {
                '+' => Token::new(TokenKind::Plus, "+".to_string()),
                '-' => Token::new(TokenKind::Minus, "-".to_string()),
                '*' => Token::new(TokenKind::Star, "*".to_string()),
                '/' => Token::new(TokenKind::Slash, "/".to_string()),
                '!' => {
                    if self.peek(0) == '=' {
                        self.advance();
                        Token::new(TokenKind::BangEqual, "!=".to_string())
                    } else {
                        Token::new(TokenKind::Bang, "!".to_string())
                    }
                }
                '=' => {
                    if self.peek(0) == '=' {
                        self.advance();
                        Token::new(TokenKind::EqualEqual, "==".to_string())
                    } else {
                        Token::new(TokenKind::Equal, "&".to_string())
                    }
                }
                '&' => {
                    if self.peek(0) == '&' {
                        self.advance();
                        Token::new(TokenKind::AmpersandAmpersand, "&&".to_string())
                    } else {
                        Token::new(TokenKind::Ampersand, "&".to_string())
                    }
                }
                '|' => {
                    if self.peek(0) == '|' {
                        self.advance();
                        Token::new(TokenKind::PipePipe, "||".to_string())
                    } else {
                        Token::new(TokenKind::Pipe, "|".to_string())
                    }
                }

                '(' => Token::new(TokenKind::OpenParen, "(".to_string()),
                ')' => Token::new(TokenKind::CloseParen, ")".to_string()),

                char => {
                    if char.is_ascii_whitespace() {
                        let start = self.current - 1;
                        while self.peek(0).is_ascii_whitespace() {
                            self.advance();
                        }
                        Token::new(
                            TokenKind::Whitespace,
                            self.source[start..self.current].iter().collect(),
                        )
                    } else if char.is_ascii_digit() {
                        let start = self.current - 1;
                        while self.peek(0).is_ascii_digit() {
                            self.advance();
                        }
                        Token::new(
                            TokenKind::Number,
                            self.source[start..self.current].iter().collect(),
                        )
                    } else if char.is_ascii_alphabetic() {
                        let start = self.current - 1;
                        while self.peek(0).is_ascii_alphanumeric() {
                            self.advance();
                        }
                        let lexeme = self.source[start..self.current].iter().collect::<String>();
                        let kind = TokenKind::get_lexeme_type(&lexeme);
                        Token::new(kind, lexeme)
                    } else {
                        self.diagnostics
                            .push(format!("Unexpected character '{char}'"));
                        Token::new(TokenKind::Invalid, char.to_string())
                    }
                }
            }
        }
    }

    fn peek(&self, offset: usize) -> char {
        let index = offset + self.current;
        if index < self.source.len() {
            self.source[index]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn next_char(&mut self) -> char {
        let char = self.peek(0);
        self.advance();
        char
    }
}