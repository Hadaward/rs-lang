#[derive(Clone, Debug)]
pub struct LexerCursor {
    pub input: String,
    pub current_char: char,
    pub position: usize,
    pub line: usize,
    pub column: usize,
    pub eof: bool
}

impl LexerCursor {
    pub fn new(input: String) -> Self {
        Self {
            input: input.clone(),
            current_char: '\0',
            position: 0,
            line: 0,
            column: 0,
            eof: input.is_empty()
        }
    }

    pub fn reset_cursor(&mut self) {
        self.position = 0;
        self.line = 0;
        self.column = 0;
        self.eof = self.input.is_empty();
        self.current_char = '\0';
    }

    pub fn next(&mut self) -> Self {
        if self.eof {
            return self.clone();
        }
    
        if self.line == 0 {
            self.line += 1;
        } else {
            self.position += 1;
        }
    
        // Usa a contagem de caracteres para evitar inconsistências com caracteres multibyte
        let total_chars = self.input.chars().count();
        if self.position >= total_chars {
            self.eof = true;
            // Garante que o índice permaneça em um valor válido
            if self.position > 0 {
                self.position -= 1;
            }
            return self.clone();
        }
    
        self.column += 1;
        self.current_char = self.input.chars().nth(self.position).unwrap();
    
        if self.current_char == '\n' {
            self.column = 1;
            self.line += 1;
        }
    
        self.clone()
    }    

    pub fn next_skip_whitespace(&mut self) -> Self {
        let total_chars = self.input.chars().count();
        while self.position + 1 < total_chars {
            if let Some(next_char) = self.input.chars().nth(self.position + 1) {
                if !next_char.is_whitespace() {
                    break;
                }
            } else {
                break;
            }
            self.next();
        }
        self.next()
    }    

    pub fn previous(&mut self) -> Self {
        if self.position == 0 {
            return self.clone();
        }
    
        self.position -= 1;
        self.current_char = self.input.chars().nth(self.position).unwrap();
        
        if self.current_char == '\n' {
            self.line -= 1;
            self.column = self.input[..self.position]
                .chars()
                .rev()
                .take_while(|&c| c != '\n')
                .count();
        } else if self.column > 1 {
            self.column -= 1;
        }
        
        if self.eof {
            self.eof = false;
        }

        self.clone()
    }

    pub fn next_is(&self, expected: char) -> bool {
        self.position + 1 < self.input.len() && self.input.chars().nth(self.position + 1) == Some(expected)
    }

    pub fn previous_is(&self, expected: char) -> bool {
        self.position > 0 && self.input.chars().nth(self.position - 1) == Some(expected)
    }
}