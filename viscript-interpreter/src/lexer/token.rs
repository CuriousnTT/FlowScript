#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Number(String),
    StringLiteral(String),
    Operator(String),
    ComparisonOperator(String),
    LogicalOperator(String),
    Punctuation(String),
    Directive(String),
    EOF,
}