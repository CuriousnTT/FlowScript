use regex::Regex;

#[derive(Debug)]
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

// Besides Keyword, this comes from AI. May be incomplete
const TOKEN_REGEX: &[(&str, Option<TokenType>)] = &[
    (r"[ \t\n]+", None), // Skip whitespace
    (r"as|async|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|false|finally|for|from|function|get|if|import|implements|in|instanceof|interface|let|meta|new|null|of|package|private|protected|public|rel|return|set|static|super|switch|target|this|throw|true|try|typeof|void|while|with|yield",
        Some(TokenType::Keyword)
    ),
    (r"&log", Some(TokenType::Directive)),
    (r"true|false|undefined", Some(TokenType::Literal)),
    (r"\d+(\.\d+)?", Some(TokenType::Number)),
    (r#""(?:\\.|[^\\"])*""#, Some(TokenType::StringLiteral)),
    (r"[A-Za-z_][A-Za-z0-9_]*", Some(TokenType::Identifier)),
    (r"[+\-*/=]", Some(TokenType::Operator)),
    ("==|!=|<=|>=|<|>", Some(TokenType::ComparisonOperator)),
    (r"&&|\|\|", Some(TokenType::LogicalOperator)),
    (r"[(){}\[\];,]", Some(TokenType::Punctuation)),
    (r"\.", Some(TokenType::Punctuation)),
    (r":", Some(TokenType::Punctuation)),
];

#[derive(Debug, Clone, Copy)]
enum TokenType {
    Keyword,
    Identifier,
    Literal,
    Number,
    StringLiteral,
    Operator,
    ComparisonOperator,
    LogicalOperator,
    Punctuation,
    Directive,
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < source_code.len() {
        let mut matched = false;
        
        for (regex_str, token_type) in TOKEN_REGEX {
            let regex = Regex::new(regex_str).unwrap();
            if let Some(mat) = regex.find(&source_code[pos..]) {
                if mat.start() == 0 {
                    matched = true;
                    let text = mat.as_str().to_string();
                    if let Some(token_type) = token_type {
                        let token = match token_type {
                            TokenType::Keyword => Token::Keyword(text),
                            TokenType::Identifier => Token::Identifier(text),
                            TokenType::Literal => Token::Literal(text),
                            TokenType::Number => Token::Number(text),
                            TokenType::StringLiteral => Token::StringLiteral(text),
                            TokenType::Operator => Token::Operator(text),
                            TokenType::ComparisonOperator => Token::ComparisonOperator(text),
                            TokenType::LogicalOperator => Token::LogicalOperator(text),
                            TokenType::Punctuation => Token::Punctuation(text),
                            TokenType::Directive => Token::Directive(text),
                        };
                        tokens.push(token);

                    };
                    pos += mat.end();
                    break;
                }
            }
        }
        if !matched {
            panic!("Unexpected character: {:?}", &source_code[pos..].chars().next().unwrap());
        }
    }
    tokens.push(Token::EOF);
    tokens
}