#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    VariableDeclaration {
        keyword: String,
        Identifier: String,
        value: Box<ASTNode>,
    },
    FunctionCall {
        callee: String,
        arguments: Vec<ASTNode>,
    },
    Block(Vec<ASTNode>),
    Expression {
        left:Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    IfStatement {
        condition: Box<ASTNode>,
        consequence: Box<ASTNode>,
    },
}