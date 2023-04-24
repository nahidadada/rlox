use std::collections::HashMap;

use crate::token::Token;
use crate::token_type::TokenType::{*, self};
use crate::token::Tokenliteral;
use crate::errors::Log;

pub struct Scanner<'a> {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    errors: &'a mut Log,
}

impl Scanner<'_> {
    pub fn new<'a>(s: &'a str, log: &'a mut Log) -> Scanner<'a> {
        let mut ks = HashMap::new();
        ks.insert("and".to_string(), And);
        ks.insert("class".to_string(), Class);
        ks.insert("else".to_string(), Else);
        ks.insert("false".to_string(), False);
        ks.insert("for".to_string(), For);
        ks.insert("fun".to_string(), Fun);
        ks.insert("if".to_string(), If);
        ks.insert("nil".to_string(), Nils);
        ks.insert("or".to_string(), Or);
        ks.insert("print".to_string(), Print);
        ks.insert("return".to_string(), Return);
        ks.insert("super".to_string(), Super);
        ks.insert("this".to_string(), This);
        ks.insert("true".to_string(), True);
        ks.insert("var".to_string(), Var);
        ks.insert("while".to_string(), While);

        Scanner {
            source: s.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: ks,
            errors: log,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let tok = Token::new(Eofs, "", &Tokenliteral::Nil, self.line);
        self.tokens.push(tok);
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(BangEqual);
                } else {
                    self.add_token(Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            '/'=> {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }                    
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.handle_string();
            }
            '0'..='9' => {
                self.handle_number();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.handle_identifier();
            }
            _ => {
                self.errors.error(self.line, "unexpeded char");
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.real_add_token(token_type, &Tokenliteral::Nil);
    }

    fn real_add_token(&mut self, token_type: TokenType, literal: &Tokenliteral) {
        let sub = &self.source[self.start..self.current];
        let text = String::from_iter(sub.iter());
        self.tokens.push(Token::new(token_type, &text, literal, self.line));
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != c {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.error(self.line, "unterminated string");
            return;
        }

        self.advance();
        let chars = &self.source[self.start + 1..self.current - 1];
        let s = String::from_iter(chars.iter());
        let literal = Tokenliteral::Lstirng(s);
        self.real_add_token(Strings, &literal);
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let v = &self.source[self.start..self.current];
        let s = String::from_iter(v.iter());
        let f = s.parse::<f64>().unwrap();
        let literal = Tokenliteral::Lnumber(f);
        self.real_add_token(Number, &literal);
    }

    fn handle_identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let ret = self.keywords.get(&text);
        if ret.is_none() {
            self.add_token(Identifier);
        } else {
            self.add_token(ret.copied().unwrap());
        }
    } 

    fn is_alpha(&self, c: char) -> bool {
        let ret = match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                true
            }
            _ => {
                false
            }
        };
        ret
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || c.is_ascii_digit();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1];
    }
}