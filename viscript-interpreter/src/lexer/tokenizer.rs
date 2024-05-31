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
const TOKEN_REGEX: &[(&str, Option<fn(String) -> Token>)] = &[
    (r"[ \t\n]+", None), // Skip whitespace
    (r"//.*", None), // Skip single-line comments
    (r"/\*[\s\S]*?\*/", None), // Skip multi-line comments
    (
        r"as|async|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|false|finally|for|from|function|get|if|import|implements|in|instanceof|interface|let|meta|new|null|of|package|private|protected|public|rel|return|set|static|super|switch|target|this|throw|true|try|typeof|void|while|with|yield",
        Some(Token::Keyword)
    ),
    (r"&log", Some(Token::Directive)),
    (r"true|false|undefined", Some(Token::Literal)),
    (r"\d+(\.\d+)?", Some(Token::Number)),
    (r#""(?:\\.|[^\\"])*""#, Some(Token::StringLiteral)),
    (r"[A-Za-z_][A-Za-z0-9_]*", Some(Token::Identifier)),
    (r"[+\-*/=]", Some(Token::Operator)),
    ("==|!=|<=|>=|<|>", Some(Token::ComparisonOperator)),
    (r"&&|\|\|", Some(Token::LogicalOperator)),
    (r"[(){}\[\];,]", Some(Token::Punctuation)),
    (r"\.", Some(Token::Punctuation)),
    (r":", Some(Token::Punctuation)),
];

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < source_code.len() {
        let mut matched = false;
        
        for (regex_str, token_fn) in TOKEN_REGEX {
            let regex = Regex::new(regex_str).unwrap();
            if let Some(mat) = regex.find(&source_code[pos..]) {
                if mat.start() == 0 {
                    matched = true;
                    let text = mat.as_str().to_string();
                    if let Some(token_fn) = token_fn {
                        let token = token_fn(text);
                        tokens.push(token);

                    };
                    pos += mat.end();
                    break;
                }
            }
        }
        if !matched {
            panic!(
                "Unexpected character: {:?} at position {}",
                &source_code[pos..].chars().next().unwrap(),
                pos
            );
        }
    }
    tokens.push(Token::EOF);
    tokens
}