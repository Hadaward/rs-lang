use interpreter::{ast::Parser, ast_node::ASTNode, token::{Token, TokenType}};

#[test]
fn test_ast_1_plus_2() {
    // Cria os tokens correspondentes à expressão "1 + 2"
    let tokens = vec![
        Token::new(TokenType::NUMBER, "1".to_string()),
        Token::new(TokenType::OPERATOR, "+".to_string()),
        Token::new(TokenType::NUMBER, "2".to_string()),
        // Opcional: token de fim de arquivo
        Token::new(TokenType::EOF, "".to_string()),
    ];

    // Cria o parser e processa os tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Valida o AST gerado
    match ast {
        ASTNode::BinaryOp { left, op, right } => {
            // Verifica se o nó à esquerda é o número 1
            if let ASTNode::Number(value) = *left {
                assert_eq!(value, 1);
            } else {
                panic!("Nó à esquerda não é um número");
            }

            // Verifica se o operador é '+'
            assert_eq!(op._type, TokenType::OPERATOR);
            assert_eq!(op.value, "+");

            // Verifica se o nó à direita é o número 2
            if let ASTNode::Number(value) = *right {
                assert_eq!(value, 2);
            } else {
                panic!("Nó à direita não é um número");
            }
        }
        _ => panic!("AST não representa uma operação binária"),
    }
}

#[test]
fn test_ast_num1_plus_num2() {
    // Cria os tokens correspondentes à expressão "num1 + num2"
    let tokens = vec![
        Token::new(TokenType::IDENTIFIER, "num1".to_string()),
        Token::new(TokenType::OPERATOR, "+".to_string()),
        Token::new(TokenType::IDENTIFIER, "num2".to_string()),
        // Opcional: token de fim de arquivo
        Token::new(TokenType::EOF, "".to_string()),
    ];

    // Cria o parser e processa os tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Valida o AST gerado
    match ast {
        ASTNode::BinaryOp { left, op, right } => {
            // Verifica se o nó à esquerda é o número 1
            if let ASTNode::Identifier(value) = *left {
                assert_eq!(value, "num1");
            } else {
                panic!("Nó à esquerda não é um identificador");
            }

            // Verifica se o operador é '+'
            assert_eq!(op._type, TokenType::OPERATOR);
            assert_eq!(op.value, "+");

            // Verifica se o nó à direita é o número 2
            if let ASTNode::Identifier(value) = *right {
                assert_eq!(value, "num2");
            } else {
                panic!("Nó à direita não é um identificador");
            }
        }
        _ => panic!("AST não representa uma operação binária"),
    }
}

#[test]
fn test_ast_variable_declaration() {
    // Cria os tokens correspondentes à declaração: var nome = 10;
    let tokens = vec![
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "nome".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "10".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        // Opcional: token de fim de arquivo
        Token::new(TokenType::EOF, "".to_string()),
    ];

    // Cria o parser e processa os tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Valida o AST gerado
    match ast {
        ASTNode::VariableDeclaration { identifier, value } => {
            // Verifica se o identificador é "nome"
            assert_eq!(identifier, "nome");

            // Verifica se o valor da declaração é o número 10
            if let ASTNode::Number(num) = *value {
                assert_eq!(num, 10);
            } else {
                panic!("O valor da variável não é um número");
            }
        }
        _ => panic!("AST não representa uma declaração de variável"),
    }
}

#[test]
fn test_ast_complex_expression_with_numbers() {
    // Cria os tokens correspondentes à expressão "(10 * (50 / 2))"
    let tokens = vec![
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::NUMBER, "10".to_string()),
        Token::new(TokenType::OPERATOR, "*".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::NUMBER, "50".to_string()),
        Token::new(TokenType::OPERATOR, "/".to_string()),
        Token::new(TokenType::NUMBER, "2".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    // Cria o parser e processa os tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Valida o AST gerado
    match ast {
        ASTNode::BinaryOp { left, op, right } => {
            // Verifica se a operação à esquerda é 10
            if let ASTNode::Number(value) = *left {
                assert_eq!(value, 10);
            } else {
                panic!("Nó à esquerda não é o número 10");
            }

            // Verifica se o operador é "*"
            assert_eq!(op._type, TokenType::OPERATOR);
            assert_eq!(op.value, "*");

            // Verifica se a expressão à direita é a divisão
            if let ASTNode::BinaryOp { left, op, right } = *right {
                // Verifica se o nó à esquerda é 50
                if let ASTNode::Number(value) = *left {
                    assert_eq!(value, 50);
                } else {
                    panic!("Nó à esquerda da divisão não é 50");
                }

                // Verifica se o operador da divisão é "/"
                assert_eq!(op._type, TokenType::OPERATOR);
                assert_eq!(op.value, "/");

                // Verifica se o nó à direita da divisão é 2
                if let ASTNode::Number(value) = *right {
                    assert_eq!(value, 2);
                } else {
                    panic!("Nó à direita da divisão não é 2");
                }
            } else {
                panic!("Nó à direita não é uma operação de divisão");
            }
        }
        _ => panic!("AST não representa a expressão esperada"),
    }
}

#[test]
fn test_ast_complex_expression_with_identifiers() {
    // Cria os tokens correspondentes à expressão "(n1 * (50 / n2) + 10)"
    let tokens = vec![
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "n1".to_string()),
        Token::new(TokenType::OPERATOR, "*".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::NUMBER, "50".to_string()),
        Token::new(TokenType::OPERATOR, "/".to_string()),
        Token::new(TokenType::IDENTIFIER, "n2".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::OPERATOR, "+".to_string()),
        Token::new(TokenType::NUMBER, "10".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    // Cria o parser e processa os tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Valida o AST gerado
    match ast {
        ASTNode::BinaryOp { left, op, right } => {
            // Verifica se a operação à esquerda é uma operação binária
            if let ASTNode::BinaryOp { left, op, right } = *left {
                // Verifica se a operação à esquerda da multiplicação é n1
                if let ASTNode::Identifier(value) = *left {
                    assert_eq!(value, "n1");
                } else {
                    panic!("Nó à esquerda da multiplicação não é o identificador n1");
                }

                // Verifica se o operador é "*"
                assert_eq!(op._type, TokenType::OPERATOR);
                assert_eq!(op.value, "*");

                // Verifica a operação à direita da multiplicação (divisão)
                if let ASTNode::BinaryOp { left, op, right } = *right {
                    // Verifica se o nó à esquerda da divisão é 50
                    if let ASTNode::Number(value) = *left {
                        assert_eq!(value, 50);
                    } else {
                        panic!("Nó à esquerda da divisão não é 50");
                    }

                    // Verifica se o operador da divisão é "/"
                    assert_eq!(op._type, TokenType::OPERATOR);
                    assert_eq!(op.value, "/");

                    // Verifica se o nó à direita da divisão é n2
                    if let ASTNode::Identifier(value) = *right {
                        assert_eq!(value, "n2");
                    } else {
                        panic!("Nó à direita da divisão não é o identificador n2");
                    }
                } else {
                    panic!("Nó à esquerda não é uma operação de divisão");
                }
            } else {
                panic!("Nó à esquerda não é uma operação binária");
            }

            // Verifica se o operador da soma é "+"
            assert_eq!(op._type, TokenType::OPERATOR);
            assert_eq!(op.value, "+");

            // Verifica se o nó à direita da soma é 10
            if let ASTNode::Number(value) = *right {
                assert_eq!(value, 10);
            } else {
                panic!("Nó à direita da soma não é 10");
            }
        }
        _ => panic!("AST não representa a expressão esperada"),
    }
}

#[test]
fn test_ast_mixed_expression() {
    // Cria os tokens correspondentes à expressão "(n1 * (50 / n2) + 10)"
    let tokens = vec![
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "n1".to_string()),
        Token::new(TokenType::OPERATOR, "*".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::NUMBER, "50".to_string()),
        Token::new(TokenType::OPERATOR, "/".to_string()),
        Token::new(TokenType::IDENTIFIER, "n2".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::OPERATOR, "+".to_string()),
        Token::new(TokenType::NUMBER, "10".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    // Cria o parser e processa os tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Valida o AST gerado
    match ast {
        ASTNode::BinaryOp { left, op, right } => {
            // Verifica se o operador é "+"
            assert_eq!(op._type, TokenType::OPERATOR);
            assert_eq!(op.value, "+");

            // Verifica se a expressão à esquerda é a operação de multiplicação
            if let ASTNode::BinaryOp { left, op, right } = *left {
                // Verifica se a operação à esquerda da multiplicação é n1
                if let ASTNode::Identifier(value) = *left {
                    assert_eq!(value, "n1");
                } else {
                    panic!("Nó à esquerda da multiplicação não é o identificador n1");
                }

                // Verifica se o operador da multiplicação é "*"
                assert_eq!(op._type, TokenType::OPERATOR);
                assert_eq!(op.value, "*");

                // Verifica se a operação à direita da multiplicação é a operação de divisão
                if let ASTNode::BinaryOp { left, op, right } = *right {
                    // Verifica se o nó à esquerda da divisão é 50
                    if let ASTNode::Number(value) = *left {
                        assert_eq!(value, 50);
                    } else {
                        panic!("Nó à esquerda da divisão não é 50");
                    }

                    // Verifica se o operador da divisão é "/"
                    assert_eq!(op._type, TokenType::OPERATOR);
                    assert_eq!(op.value, "/");

                    // Verifica se o nó à direita da divisão é n2
                    if let ASTNode::Identifier(value) = *right {
                        assert_eq!(value, "n2");
                    } else {
                        panic!("Nó à direita da divisão não é o identificador n2");
                    }
                } else {
                    panic!("Nó à direita da multiplicação não é uma operação de divisão");
                }
            } else {
                panic!("Nó à esquerda não é uma operação de multiplicação");
            }

            // Verifica se o nó à direita da soma é 10
            if let ASTNode::Number(value) = *right {
                assert_eq!(value, 10);
            } else {
                panic!("Nó à direita da soma não é 10");
            }
        }
        _ => panic!("AST não representa a expressão esperada"),
    }
}

#[test]
fn test_ast_variable_declaration_with_complex_expression() {
    let tokens = vec![
        // var a = 10;
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "a".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "10".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // var b = 20;
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "b".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "20".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // var c = 30;
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "c".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "30".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // var d = 2 * ((a + b) * (c / 2));
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "d".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "2".to_string()),
        Token::new(TokenType::OPERATOR, "*".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "a".to_string()),
        Token::new(TokenType::OPERATOR, "+".to_string()),
        Token::new(TokenType::IDENTIFIER, "b".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::OPERATOR, "*".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "c".to_string()),
        Token::new(TokenType::OPERATOR, "/".to_string()),
        Token::new(TokenType::NUMBER, "2".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        Token::new(TokenType::EOF, "".to_string()),
    ];

    let mut parser = Parser::new(tokens);
    
    // Testa a variável 'a'
    let ast = parser.parse();
    println!("AST Gerado: {:?}", ast);
    match ast {
        ASTNode::VariableDeclaration { identifier, value } => {
            assert_eq!(identifier, "a");
            assert_eq!(*value, ASTNode::Number(10));
        }
        _ => panic!("AST não representa a declaração de variável correta"),
    }

    // Testa a variável 'b'
    let ast = parser.parse();
    println!("AST Gerado: {:?}", ast);
    match ast {
        ASTNode::VariableDeclaration { identifier, value } => {
            assert_eq!(identifier, "b");
            assert_eq!(*value, ASTNode::Number(20));
        }
        _ => panic!("AST não representa a declaração de variável correta"),
    }

    // Testa a variável 'c'
    let ast = parser.parse();
    println!("AST Gerado: {:?}", ast);
    match ast {
        ASTNode::VariableDeclaration { identifier, value } => {
            assert_eq!(identifier, "c");
            assert_eq!(*value, ASTNode::Number(30));
        }
        _ => panic!("AST não representa a declaração de variável correta"),
    }

    // Testa a variável 'd'
    let ast = parser.parse();
    println!("AST Gerado: {:?}", ast);
    match ast {
        ASTNode::VariableDeclaration { identifier, value } => {
            assert_eq!(identifier, "d");
            
            if let ASTNode::BinaryOp { left, op, right } = *value {
                // Verifica o operador '*'
                assert_eq!(op.value, "*");

                // Verifica a estrutura do "left" (deveria ser uma operação binária com a soma de a e b)
                if let ASTNode::BinaryOp { left, op, right } = *left {
                    // Verifica o operador '+'
                    assert_eq!(op.value, "+");

                    if let ASTNode::Identifier(identifier) = *left {
                        assert_eq!(identifier, "a");
                    } else {
                        panic!("Esperado identificador 'a'");
                    }

                    if let ASTNode::Identifier(identifier) = *right {
                        assert_eq!(identifier, "b");
                    } else {
                        panic!("Esperado identificador 'b'");
                    }
                } else {
                    panic!("Esperado operador de soma");
                }

                // Verifica a estrutura do "right" (deveria ser uma operação binária com a divisão de c por 2)
                if let ASTNode::BinaryOp { left, op, right } = *right {
                    // Verifica o operador '/'
                    assert_eq!(op.value, "/");

                    if let ASTNode::Identifier(identifier) = *left {
                        assert_eq!(identifier, "c");
                    } else {
                        panic!("Esperado identificador 'c'");
                    }

                    if let ASTNode::Number(value) = *right {
                        assert_eq!(value, 2);
                    } else {
                        panic!("Esperado número 2");
                    }
                } else {
                    panic!("Esperado operador de divisão");
                }
            } else {
                panic!("Esperado uma operação binária para 'd'");
            }
        }
        _ => panic!("AST não representa a declaração de variável correta"),
    }
}