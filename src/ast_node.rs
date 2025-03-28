use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(i64),
    Identifier(String),
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    VariableDeclaration {
        identifier: String,
        value: Box<ASTNode>,
    },
}