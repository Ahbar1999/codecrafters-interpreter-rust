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
    pub literal:    String,
    pub line:       usize,   
}

impl Token {
    fn new(_type: TokenType, lexeme: &'source str, literal: &'source str) -> Self {
        Self {
            _type:  _type,
            lexeme: String::From(lexeme),
            literal:String::From(literal),
            line:   usize, 
        }
    }
}
