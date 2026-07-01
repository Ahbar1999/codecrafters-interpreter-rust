use crate::token::{TokenType, Token};

pub struct Scanner<'source_code> {
    source: &'source_code str,
    
    tokens: Vec<Token>,
} 

pub struct ScannerIter<'source_code> {
    source: &'source_code str,
    // scanner internal pointers
    start:  usize,
    current:usize,
    line:   usize, 
}

impl Iterator for ScannerIter {
    type Item = Token;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.source.len() {
            return None;
        }

        let result = match self.source[current] {
            '(' => {
                Some(Token::new(TokenType::LeftBrace, &self.source[start..current], "", self.line))
            },
            ')' => {
                Some(Token::new(TokenType::RightBrace, &self.source[start..current], "", self.line))
            },
            '{' => {
                Some(Token::new(TokenType::LeftBrace, &self.source[start..current], "", self.line))
            },
            '}' => {
                Some(Token::new(TokenType::RightBrace, &self.source[start..current], "", self.line))
            },
            ',' => {
                Some(Token::new(TokenType::COMMA, &self.source[start..current], "", self.line))
            },
            '.' => {
                Some(Token::new(TokenType::DOT, &self.source[start..current], "", self.line))
            },
            '-' => {
                Some(Token::new(TokenType::MINUS, &self.source[start..current], "", self.line))
            },
            '+' => {
                Some(Token::new(TokenType::PLUS, &self.source[start..current], "", self.line))
            },
            ';' => {
                Some(Token::new(TokenType::SemiColon, &self.source[start..current], "", self.line))
            },
            '*' => {
                Some(Token::new(TokenType::STAR, &self.source[start..current], "", self.line))
            },
            _ => {
                panic!("Unidentified lexeme");
            }
        };

        self.current += 1;

        result
    }
}

impl Scanner {
    pub fn new(source: &'source_code str) -> Self {
        Self {
            source,
            tokens: vec![],
        }
    }

    // consumes self
    pub fn scan_tokens(mut self) -> Vec<Token> {
    
        for token in ScannerIter::new(self.source) {
            self.tokens.push(token);
        }
 
        return self.tokens;
    }

    
}

impl ScannerIter {

    pub fn new(source: &'source_code str) -> Self {
        Self {
            source,
            start: 0, 
            current: 0, 
            line: 1
        }
    } 
}


