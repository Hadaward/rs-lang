use std::error::Error;

use crate::{lexer_cursor::LexerCursor, token::{Token, TokenType}};

const KEYWORDS: &[&str] = &["função", "var", "para", "retorne"];

pub struct Lexer {
    cursor: LexerCursor
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            cursor: LexerCursor::new(input)
        }
    }

    pub fn is_keyword(&self, value: String) -> bool {
        return KEYWORDS.contains(&value.as_str());
    }

    fn push_token(&mut self, _type: TokenType, value: char, tokens: &mut Vec<Token>) {
        tokens.push(Token::new(_type, value.to_string()));
    }

    pub fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while !self.cursor.eof && (self.cursor.current_char.is_alphanumeric() || self.cursor.current_char == '_') {
            identifier.push(self.cursor.current_char);
            self.cursor.next();
        }

        self.cursor.previous();

        return Token::new(TokenType::IDENTIFIER, identifier);
    }

    pub fn read_number(&mut self) -> Token {
        let mut number = String::new();

        while !self.cursor.eof && (self.cursor.current_char.is_digit(10) || self.cursor.current_char == '.' || self.cursor.current_char == '_') {
            if self.cursor.current_char != '_' {
                number.push(self.cursor.current_char);
            }
            self.cursor.next();
        }

        self.cursor.previous();

        return Token::new(TokenType::NUMBER, number);
    }

    pub fn read_string(&mut self) -> Result<Token, Box<dyn Error>> {
        let mut string = String::new();
        let delim = self.cursor.current_char;

        let initial_line = self.cursor.line;
        let initial_column = self.cursor.column;

        self.cursor.next();

        while !self.cursor.eof && self.cursor.current_char != delim {
            string.push(self.cursor.current_char);
            self.cursor.next();
        }

        if self.cursor.current_char != delim {
            return Err(format!(
                "Erro: esperado fechamento da string ({}) iniciada na linha {} e coluna {} antes do fim do arquivo.",
                delim, initial_line, initial_column
            ).into());
        }

        Ok(Token::new(TokenType::STRING, string))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        if self.cursor.eof {
            return Ok(Vec::new());
        }

        let mut tokens: Vec<Token> = Vec::new();
        let mut current_cursor = self.cursor.next_skip_whitespace();

        while !current_cursor.eof {
            match current_cursor.current_char {
                '=' => {
                    if current_cursor.previous_is('+') || current_cursor.previous_is('-') || current_cursor.previous_is('*') || current_cursor.previous_is('/') {
                        let mut previous_token = tokens.pop().unwrap();
                        previous_token.value.push(current_cursor.current_char);

                        tokens.push(Token::new(TokenType::ASSIGNMENT, previous_token.value));
                    } else {
                        self.push_token(TokenType::ASSIGNMENT, current_cursor.current_char, &mut tokens);
                    }
                },
                '+' => self.push_token(TokenType::OPERATOR, current_cursor.current_char, &mut tokens),
                '-' => self.push_token(TokenType::OPERATOR, current_cursor.current_char, &mut tokens),
                '*' => self.push_token(TokenType::OPERATOR, current_cursor.current_char, &mut tokens),
                '/' => self.push_token(TokenType::OPERATOR, current_cursor.current_char, &mut tokens),
                '(' => self.push_token(TokenType::LPAREN, current_cursor.current_char, &mut tokens),
                ')' => self.push_token(TokenType::RPAREN, current_cursor.current_char, &mut tokens),
                '{' => self.push_token(TokenType::LBRACKET, current_cursor.current_char, &mut tokens),
                '}' => self.push_token(TokenType::RBRACKET, current_cursor.current_char, &mut tokens),
                ',' => self.push_token(TokenType::COMMA, current_cursor.current_char, &mut tokens),
                ';' => self.push_token(TokenType::SEMICOLON, current_cursor.current_char, &mut tokens),
                '<' => self.push_token(TokenType::OPERATOR, current_cursor.current_char, &mut tokens),
                '>' => self.push_token(TokenType::OPERATOR, current_cursor.current_char, &mut tokens),
                _ => {
                    if current_cursor.current_char.is_alphabetic() || current_cursor.current_char == '_' {
                        let mut token = self.read_identifier();
    
                        if self.is_keyword(token.value.clone()) {
                            token._type = TokenType::KEYWORD;
                        }
    
                        tokens.push(token);
                    } else if current_cursor.current_char.is_digit(10) {
                        tokens.push(self.read_number());
                    } else if current_cursor.current_char == '"' || current_cursor.current_char == '\'' {
                        let result = self.read_string();
                        
                        if result.is_err() {
                            return Err(result.unwrap_err());
                        }
    
                        tokens.push(result.unwrap());
                    } else {
                        return Err(format!(
                            "Caractere inválido encontrado '{}' na linha {} e coluna {}",
                            current_cursor.current_char,
                            current_cursor.line,
                            current_cursor.column
                        ).into());
                    }
                }
            }
    
            current_cursor = self.cursor.next_skip_whitespace();
        }

        Ok(tokens)
    }
}