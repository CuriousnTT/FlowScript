use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

use super::token::Token;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexpected character: '{}' at position {}", c, pos)
            }
        }
    }
}

impl std::error::Error for LexerError {}

// Besides Keyword, this comes from AI. May be incomplete
lazy_static! {
    static ref TOKEN_REGEX: Vec<(Regex, Option<fn(String) -> Token>)> = vec![
        (Regex::new(r"[ \t\n]+").unwrap(), None), // Skip whitespace
        (Regex::new(r"//.*").unwrap(), None), // Skip single-line comments
        (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), None), // Skip multi-line comments
        (
            Regex::new(r"as|async|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|false|finally|for|from|function|get|if|import|implements|in|instanceof|interface|let|meta|new|null|of|package|private|protected|public|rel|return|set|static|super|switch|target|this|throw|true|try|type|typeof|void|while|with|yield").unwrap(),
            Some(Token::Keyword)
        ),
        (Regex::new(r"&log").unwrap(), Some(Token::Directive)),
        (Regex::new(r"true|false|undefined").unwrap(), Some(Token::Literal)),
        (Regex::new(r"\d+(\.\d+)?").unwrap(), Some(Token::Number)),
        (Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap(), Some(Token::StringLiteral)),
        (Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap(), Some(Token::Identifier)),
        (Regex::new(r"[+\-*/=]").unwrap(), Some(Token::Operator)),
        (Regex::new("==|!=|<=|>=|<|>").unwrap(), Some(Token::ComparisonOperator)),
        (Regex::new(r"&&|\|\|").unwrap(), Some(Token::LogicalOperator)),
        (Regex::new(r"[(){}\[\];,]").unwrap(), Some(Token::Punctuation)),
        (Regex::new(r"\.").unwrap(), Some(Token::Punctuation)),
        (Regex::new(r":").unwrap(), Some(Token::Punctuation)),
    ];
}

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < source_code.len() {
        let mut matched = false;
        
        for (regex, token_fn) in TOKEN_REGEX.iter() {
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
            return Err(LexerError::UnexpectedCharacter(
                source_code[pos..].chars().next().unwrap(), pos
            ));
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}