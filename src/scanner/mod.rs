use crate::token::Token;

pub struct Scanner<'source_code> {
    source: &'source_code str,
    tokens: Vec<Token>,

    // scanner internal pointers
    start:  usize,
    current:usize,
    line:   usize,
} 

impl Scanner {
    pub fn new(source: &'source_code str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn scan_tokens() -> Vec<Token> {
        unimplemented!()
    } 
}
