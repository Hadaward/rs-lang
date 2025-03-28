use interpreter::{lexer::Lexer, token::{Token, TokenType}};

const INPUT: &str = r#"
    var x = 1;
    função soma(n1, n2) {
        var resultado = n1 + n2;
        retorne resultado;
    }
    para (var índice = 0; índice < 10; índice += 1) {
        x = soma(x, índice);
        imprimir(x);
    }

    var y = 'hello world';
    imprimir(y);
"#;

#[test]
fn check_lexer_tokenize_fn() {
    let code = String::from(INPUT);
    let mut lexer = Lexer::new(code.clone());

    let result = lexer.tokenize();
    assert!(result.is_ok(), "Lexer falhou: {:?}", result.err());
    let tokens = result.unwrap();

    let expected_tokens = vec![
        // var x = 1;
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "x".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "1".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // função soma(n1, n2) {
        Token::new(TokenType::KEYWORD, "função".to_string()),
        Token::new(TokenType::IDENTIFIER, "soma".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "n1".to_string()),
        Token::new(TokenType::COMMA, ",".to_string()),
        Token::new(TokenType::IDENTIFIER, "n2".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::LBRACKET, "{".to_string()),

        // var resultado = n1 + n2;
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "resultado".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::IDENTIFIER, "n1".to_string()),
        Token::new(TokenType::OPERATOR, "+".to_string()),
        Token::new(TokenType::IDENTIFIER, "n2".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // retorne resultado;
        Token::new(TokenType::KEYWORD, "retorne".to_string()),
        Token::new(TokenType::IDENTIFIER, "resultado".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // Fechamento da função: }
        Token::new(TokenType::RBRACKET, "}".to_string()),

        // para (var índice = 0; índice < 10; índice += 1) {
        Token::new(TokenType::KEYWORD, "para".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "índice".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::NUMBER, "0".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::IDENTIFIER, "índice".to_string()),
        Token::new(TokenType::OPERATOR, "<".to_string()),
        Token::new(TokenType::NUMBER, "10".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::IDENTIFIER, "índice".to_string()),
        Token::new(TokenType::ASSIGNMENT, "+=".to_string()),
        Token::new(TokenType::NUMBER, "1".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),

        // Abertura do bloco do laço para
        Token::new(TokenType::LBRACKET, "{".to_string()),

        // x = soma(x, índice);
        Token::new(TokenType::IDENTIFIER, "x".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::IDENTIFIER, "soma".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "x".to_string()),
        Token::new(TokenType::COMMA, ",".to_string()),
        Token::new(TokenType::IDENTIFIER, "índice".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // imprimir(x);
        Token::new(TokenType::IDENTIFIER, "imprimir".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "x".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // Fechamento do bloco do laço para: }
        Token::new(TokenType::RBRACKET, "}".to_string()),

        // var y = "hello world";
        Token::new(TokenType::KEYWORD, "var".to_string()),
        Token::new(TokenType::IDENTIFIER, "y".to_string()),
        Token::new(TokenType::ASSIGNMENT, "=".to_string()),
        Token::new(TokenType::STRING, "hello world".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),

        // imprimir(y);
        Token::new(TokenType::IDENTIFIER, "imprimir".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENTIFIER, "y".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
    ];

    assert_eq!(tokens.len(), expected_tokens.len(), "Número inesperado de tokens.");

    // Verifica se os tokens gerados são os mesmos que os esperados
    for (expected_token, token) in expected_tokens.iter().zip(tokens.iter()) {
        assert_eq!(
            expected_token._type,
            token._type,
            "Erro no token: Expected {:?}, found {:?}",
            expected_token,
            token
        );
        assert_eq!(
            expected_token.value,
            token.value,
            "Erro no valor do token: Expected '{}', found '{}'",
            expected_token.value,
            token.value
        );
    }
}

#[test]
fn check_lexer_invalid_characters() {
    let invalid_input = "var x = 1 + $;";  // '$' é um caractere inválido
    let mut lexer = Lexer::new(invalid_input.to_string());

    let result = lexer.tokenize();

    assert!(
        result.is_err(),
        "Lexer não detectou erro para caractere inválido"
    );
}

#[test]
fn check_lexer_assignment_compound() {
    let code = "var x = 10; x += 5;";
    let mut lexer = Lexer::new(code.to_string());

    let result = lexer.tokenize();
    assert!(result.is_ok(), "Lexer falhou: {:?}", result.err());

    let tokens = result.unwrap();

    // A sequência de tokens para "var x = 10; x += 5;" deverá ser:
    // [0] KEYWORD "var"
    // [1] IDENTIFIER "x"
    // [2] ASSIGNMENT "="
    // [3] NUMBER "10"
    // [4] SEMICOLON ";"
    // [5] IDENTIFIER "x"
    // [6] ASSIGNMENT "+="   <-- este é o token composto
    // [7] NUMBER "5"
    // [8] SEMICOLON ";"
    assert_eq!(tokens[6]._type, TokenType::ASSIGNMENT);
    assert_eq!(tokens[6].value, "+=");
}