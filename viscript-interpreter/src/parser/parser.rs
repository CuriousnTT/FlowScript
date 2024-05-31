use super::super::lexer::tokenizer;
use parser::ast::ASTNode;
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: &[Token]) -> ASTNode {
    let mut tokens = tokens.iter().peekable();
    parse_program(&mut tokens);
}

fn parse_program(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    let mut nodes = Vec::new();
    while let Some(_) = tokens.peek() {
        nodes.push(parse_statement(tokens));
    }
    ASTNode::Program(nodes)
}

fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    match tokens.peek() {
        Some(Token::Keyword(kw)) if kw == "let" || kw == "const" || kw == "rel" => {
            parse_variable_declaration(tokens)
        }
        Some(Token::Keyword(kw)) if kw == "function" => parse_function_declaration(tokens),
        Some(Token::Identifier(_)) => parse_expression_statement(tokens),
        Some(Token::Keyword(kw)) if kw == "if" => parse_if_statement(tokens),
        _ => panic!("Unexpected token: {:?}", tokens.peek()),
    }
}

fn parse_variable_declaration(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    //TODO: Implement reactivity for 'let' and 'rel'
    let keyword = match tokens.next() {
        Some(Token::Keyword(kw)) => kw.clone(),
        _ => panic!("Expected a keyeword"),
    };

    let identifier = match tokens.next() {
        Some(Token::Identifier(id)) => id.clone(),
        _ => panic!("Expected an identifier"),
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => (),
        _ => panic!("Expected '='"),
    }

    let value = parse_expression(tokens);

    match tokens.next() {
        Some(Token::Punctuation(p)) if p == ";" => (),
        _ => panic!("Expected ';'"),
    }

    ASTNode::VariableDeclaration {
        keyword,
        identifier,
        value: Box::new(value),
    }
}

//TODO: parse everything that isn't a variable declaration