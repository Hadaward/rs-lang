use crate::{ast_node::ASTNode, token::{Token, TokenType}};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    // Construtor do parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }
    
    // Retorna o token atual sem avançar
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // Avança para o próximo token
    fn advance(&mut self) {
        self.position += 1;
    }

    // Função principal de parse que retorna o AST
    pub fn parse(&mut self) -> ASTNode {
        if let Some(token) = self.current_token().cloned() {
            if token._type == TokenType::KEYWORD && token.value == "var" {
                return self.parse_variable_declaration();
            }
        }
        self.parse_expression()
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        // Consome o token "var"
        self.advance();

        // Espera um identificador após "var"
        let id_token = self.current_token().cloned()
            .expect("Esperava um identificador após 'var'");
        if id_token._type != TokenType::IDENTIFIER {
            panic!("Esperava um identificador, encontrou: {:?}", id_token);
        }
        self.advance();

        // Espera o token de atribuição "="
        let assign_token = self.current_token().cloned()
            .expect("Esperava '=' após o identificador");
        if assign_token._type != TokenType::ASSIGNMENT {
            panic!("Esperava '=', encontrou: {:?}", assign_token);
        }
        self.advance();

        // Interpreta a expressão do lado direito do '='
        let expr = self.parse_expression();

        // Espera um ponto e vírgula ao final da declaração
        let semicolon_token = self.current_token().cloned()
            .expect("Esperava ';' ao final da declaração");
        if semicolon_token._type != TokenType::SEMICOLON {
            panic!("Esperava ';', encontrou: {:?}", semicolon_token);
        }
        self.advance();

        ASTNode::VariableDeclaration {
            identifier: id_token.value,
            value: Box::new(expr),
        }
    }

    // Analisa uma expressão que, neste caso, pode ser uma operação binária
    fn parse_expression(&mut self) -> ASTNode {
        // Começamos com a análise de termos, considerando a precedência de operadores
        let mut left = self.parse_term();

        // Analisa os operadores de soma e subtração
        while let Some(token) = self.current_token().cloned() {
            if token._type == TokenType::OPERATOR && (token.value == "+" || token.value == "-") {
                let op = token.clone();
                self.advance();
                let right = self.parse_term();
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        left
    }

    // Processa um "termo": pode ser um número ou um identificador
    fn parse_term(&mut self) -> ASTNode {
        let mut left = self.parse_factor();

        // Analisa multiplicação e divisão
        while let Some(token) = self.current_token().cloned() {
            if token._type == TokenType::OPERATOR && (token.value == "*" || token.value == "/") {
                let op = token.clone();
                self.advance();
                let right = self.parse_factor();
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        left
    }

    fn parse_factor(&mut self) -> ASTNode {
        if let Some(token) = self.current_token().cloned() {
            match token._type {
                TokenType::NUMBER => {
                    self.advance();
                    let value = token.value.parse::<i64>().expect("Número inválido");
                    ASTNode::Number(value)
                },
                TokenType::IDENTIFIER => {
                    self.advance();
                    ASTNode::Identifier(token.value)
                },
                TokenType::LPAREN => {
                    // Abre parênteses, parse a expressão interna
                    self.advance();
                    let expr = self.parse_expression();
    
                    // Espera o fechamento do parêntese
                    if let Some(token) = self.current_token().cloned() {
                        if token._type == TokenType::RPAREN {
                            self.advance();
                            expr
                        } else {
                            panic!("Esperava ')', encontrou: {:?}", token);
                        }
                    } else {
                        panic!("Esperava ')', mas não encontrou");
                    }
                },
                _ => panic!("Token inesperado: {:?}", token),
            }
        } else {
            panic!("Fim dos tokens inesperado");
        }
    }
}