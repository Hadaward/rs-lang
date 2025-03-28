#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    KEYWORD,
    IDENTIFIER,
    NUMBER,
    STRING,
    ASSIGNMENT,
    OPERATOR,
    SEMICOLON,
    COMMA,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    EOF
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub _type: TokenType,
    pub value: String
}

impl Token {
    pub fn new(_type: TokenType, value: String) -> Self {
        Self {
            _type: _type.clone(),
            value: value.clone()
        }
    }
}