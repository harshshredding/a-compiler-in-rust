use std::collections::HashMap;
use crate::lexical_analysis::Token;
use super::lexical_analysis::TokenType;
use super::lexical_analysis::Scanner;
use std::fs::File;
use std::io::Write;

pub fn get_calgary_token(token: TokenType) -> String {
    match token {
        TokenType::Function => String::from("function"),
        TokenType::Identifier => String::from("id"),
        TokenType::OpenParenthesis => String::from("lpar"),
        TokenType::CloseParenthesis => String::from("rpar"),
        TokenType::Colon => String::from("colon"),
        TokenType::IntegerKeyword => String::from("integer"),
        TokenType::OpenSquareBracket => String::from("lsqbr"),
        TokenType::CloseSquareBracket => String::from("rsqbr"),
        TokenType::Comma => String::from("comma"),
        TokenType::LocalVar => String::from("localvar"),
        TokenType::SemiColon => String::from("semi"),
        TokenType::Arrow => String::from("arrow"),
        TokenType::Void => String::from("void"),
        TokenType::OpenCurly => String::from("lcurbr"),
        TokenType::CloseCurly => String::from("rcurbr"),
        TokenType::EqualsSymbol => String::from("equal"),
        TokenType::IntLit => String::from("intlit"),
        TokenType::While => String::from("while"),
        TokenType::LessThan => String::from("lt"),
        TokenType::GreaterThan => String::from("gt"),
        TokenType::Minus => String::from("minus"),
        TokenType::If => String::from("if"),
        TokenType::Plus => String::from("plus"),
        TokenType::Then => String::from("then"),
        TokenType::Else => String::from("else"),
        TokenType::Write => String::from("write"),
        TokenType::Read => String::from("read"),
        TokenType::Class => String::from("class"),
        TokenType::Public => String::from("public"),
        TokenType::Private => String::from("private"),
        TokenType::FloatKeyword => String::from("float"),
        TokenType::IsA => String::from("isa"),
        TokenType::Attribute => String::from("attribute"),
        TokenType::Constructor => String::from("constructor"),
        TokenType::Sr => String::from("sr"),
        TokenType::Return => String::from("return"),
        TokenType::Asterix => String::from("mult"),
        TokenType::Period => String::from("dot"),
        TokenType::FloatLit => String::from("floatlit"),
        TokenType::LessThanOrEq => String::from("leq"),
        TokenType::GreaterThanOrEq => String::from("geq"),
        _ => panic!("This token type has not been mapped")
    }
}


pub fn parser_helper(
    table_dict: &HashMap<String, HashMap<String, String>>,
    mut calgary_tokens: Vec<String>,
    curr_non_terminal: String,
    head: Vec<String>,
    tail: Vec<String>,
    terminal_list: &Vec<String>,
    mut file: &File,
) {
    let productions_dict = &table_dict[&curr_non_terminal];
    let production = &productions_dict[&calgary_tokens[0]];
    
}