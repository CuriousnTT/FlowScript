use super::super::lexer::token::Token;

#[derive(Debug)]
pub enum ASTNode {
    VariableDeclaration {
        keyword: Token,
        identifier: String,
        value: Box<ASTNode>,
        directive: Option<Token>,
    },
    FunctionDeclaration {
        name: String,
        body: Box<ASTNode>,
    },
    /*FunctionCall {
        callee: String,
        arguments: Vec<ASTNode>,
    },*/
    Expression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    MemberAccess {
        object: Box<ASTNode>,
        property: String,
    },
    IfStatement {
        condition: Box<ASTNode>,
        consequence: Box<ASTNode>,
    },
    Block(Vec<ASTNode>),
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    Directive(String),
    Program(Vec<ASTNode>),
}