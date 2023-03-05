use std::collections::HashMap;
use crate::lexical_analysis::Token;
use super::lexical_analysis::TokenType;
use super::lexical_analysis::Scanner;
use std::fs::File;
use std::{fs, io};
use std::io::{BufRead, Write};

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


pub fn write_production(
    derived_parts: &Vec<String>,
    production_parts: &Vec<&str>,
    focus_idx: usize,
    head: &Vec<String>,
    tail: &Vec<String>,
    mut output_file: &File
) {
    let mut ret = String::from("START -> ");
    ret.push_str(&head.join(" "));
    ret.push_str(" ");
    ret.push_str(&derived_parts.join(" "));
    ret.push_str(" ");
    ret.push_str(&format!("*{}*", production_parts[focus_idx]));
    ret.push_str(" ");
    ret.push_str(&production_parts[(focus_idx + 1)..].join(" "));
    ret.push_str(" ");
    ret.push_str(&tail.join(" "));
    ret.push_str("\n");
    output_file.write_all(ret.as_bytes()).expect(&format!("Failed to write to file: {}", &ret))
}

pub fn parser_helper(
    table_dict: &HashMap<String, HashMap<String, String>>,
    calgary_tokens: &mut Vec<String>,
    curr_non_terminal: String,
    head: Vec<String>,
    tail: Vec<String>,
    terminal_list: &Vec<String>,
    output_file: &File)
    -> Vec<String>
{
    let productions_dict = table_dict.get(&curr_non_terminal)
        .unwrap_or_else(|| panic!("Not able to find non-terminal {}", &curr_non_terminal));
    let production = &productions_dict[&calgary_tokens[0]];
    let production_parts: Vec<&str> = production.split_whitespace().collect();
    assert!(production_parts.len() >= 3, "We need at least 3 elements in a production.\
                                          Production {}", production.as_str());
    assert_eq!(production_parts[0], curr_non_terminal.as_str());
    assert_eq!(production_parts[1], "→");
    assert!(production_parts[0].chars().all(is_uppercase_or_number),
            "Non terminals should be UPPERCASE OR NUMERIC {}", production_parts[0]);
    let mut derived_parts: Vec<String> = vec![];
    for (focus_idx, production_element) in production_parts[2..].iter().enumerate() {
        write_production(
            &derived_parts,
            &production_parts,
            focus_idx + 2,
            &head,
            &tail,
            output_file
        );
        if production_element.chars().all(char::is_lowercase) || (*production_element == "&epsilon") {
            // we found a terminal
            if (*production_element) != "&epsilon" {
                assert_eq!(production_element, &calgary_tokens[0]);
                calgary_tokens.remove(0);
                derived_parts.push(production_element.to_string());
            }
        } else {
            assert!(production_element.chars().all(is_uppercase_or_number),
                    "Non terminal element should be UPPERCASE OR NUMERIC {}", production_element);
            let mut new_head = head.clone();
            new_head.extend(derived_parts.clone());
            let mut new_tail: Vec<String> = production_parts[(3 + focus_idx)..].to_vec().into_iter().map(|s|s.to_string()).collect();
            new_tail.extend(tail.clone());
            let derivation = parser_helper(
                table_dict,
                calgary_tokens,
                production_element.to_string(),
                new_head,
                new_tail,
                terminal_list,
                output_file
            );
            for terminal in &derivation {
                assert!(terminal_list.contains(terminal))
            }
            derived_parts.extend(derivation);
        }
    }
    return derived_parts;
}

pub fn is_uppercase_or_number(c: char) -> bool {
    return c.is_uppercase() || c.is_numeric()
}

pub fn get_terminal_list() -> Vec<String> {
    let file = File::open("all_terminals.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let terminal_list: Vec<String> = lines.into_iter().map(|x| x.unwrap()).collect();
    return terminal_list
}

pub fn get_table_dict(table_dict_path : &str) -> HashMap<String, HashMap<String, String>> {
    let table_dict_string = fs::read_to_string(table_dict_path).unwrap();
    let table_dict: HashMap<String, HashMap<String, String>> = serde_json::from_str(&table_dict_string)
        .expect(&format!("Cannot parse json: {}", &table_dict_string));
    return table_dict
}

pub fn parse(
    table_dict: &HashMap<String, HashMap<String, String>>,
    calgary_tokens: &mut Vec<String>,
    mut output_file: &File,
) {
    calgary_tokens.push("eof".to_string());
    let terminal_list = get_terminal_list();
    parser_helper(
        table_dict,
        calgary_tokens,
        "START".to_string(),
        vec![],
        vec![],
        &terminal_list,
        output_file
    );
    output_file.write_all("\n Parsed Succesfully".as_bytes())
        .expect("Failed to write");
}