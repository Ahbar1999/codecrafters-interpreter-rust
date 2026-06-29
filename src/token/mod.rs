use crate::datum::{ Datum };

pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BangEqual,
    EQUAL, EqualEqual,
    GREATER, GreaterEqual,
    LESS, LessEqual,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

pub struct Token {
    pub _type:      TokenType,
    pub lexeme:     String,
    pub literal:    impl Datum, // this could be a literal of integer, string, float etc.
                                // monomorphised because there's a fixed and small set of datum
                                // implementers
    pub line:       usize,   
}

impl Token {
    pub fn new(_type: TokenType, lexeme: &'source str, literal: &'source str) -> Self {
        Self {
            _type:  _type,
            lexeme: String::From(lexeme),   // borrowing lexeme and literal doenst make sense
                                            // the source text is read once, it cannot live as long
                                            // as the program, we need our own copies of strings
            literal:String::From(literal),
            line:   usize, 
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self._type, self.lexeme, self.literal) 
    } 
}
