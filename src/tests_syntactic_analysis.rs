use super::lexical_analysis::*;
use super::semantic_graph::*;
use std::collections::HashMap;
use super::syntactic_analysis::*;
use std::fs::File;
use std::io::Write;
use dot::render;
use serde::{Serialize, Deserialize};
use serde::de::Unexpected::Map;

#[cfg(test)]
fn modifies_and_returns(mut some_string: String) -> String {
    some_string = some_string + " hello";
    return some_string;
}

#[test]
fn test_split_a_string() {
    let string = String::from("a → X + Y");
    let split_string: Vec<&str> = string.split_whitespace().collect();
    assert_eq!(split_string, vec!["a", "→", "X", "+", "Y"])
}

#[test]
fn test_slice() {
    let strings = vec!["hello", "world", "."];
    let slice = &strings[1..];
    assert_eq!(slice.len(), 2);
    let strings: Vec<String> = slice.into_iter().map(|x| x.to_string()).collect();
    assert_eq!(strings, vec![String::from("world"), String::from(".")]);
    for (i, el) in strings.iter().enumerate() {
        if i == 0 {
            assert_eq!(el.as_str(), "world");
        } else {
            assert_eq!(el.as_str(), ".")
        }
    }
}

#[test]
fn test_string_comparison() {
    let some_string = "hello";
    if some_string != "hello" {
        panic!("String should be equal")
    }
}

#[test]
fn test_get_terminals_list() {
    let terminals_list = get_terminal_list();
    assert_eq!(terminals_list.len(), 49);
    assert_eq!(terminals_list[3], "id".to_string())
}

#[test]
fn test_reading_json() {
    let a_dict: HashMap<String, String> = serde_json::from_str("{\"x\": \"x2\", \"y\":\"y2\"}").unwrap();
    assert_eq!(a_dict["x"], "x2");
}

#[test]
fn test_read_table_dict() {
    let table_dict = get_table_dict("src/grammars/full_table.json");
    assert_eq!(table_dict["ADDOP"].len(), 3);
    assert_eq!(table_dict["INDICE"]["lsqbr"], "INDICE \u{2192} lsqbr ARITHEXPR rsqbr");
}

#[test]
fn test_parse() {
    let source_file_content = read_source_file("src/syntax_tests/src/test.src".to_string());
    let mut scanner = Scanner::from(source_file_content);
    let all_tokens = scanner.get_all_tokens();
    let mut calgary_tokens: Vec<String> = all_tokens.into_iter()
        .map(|token| get_calgary_token(token.token_type)).collect();
    let table_dict = get_table_dict("src/grammars/full_table.json");
    let output_file = File::create("src/syntax_tests/out/test.derivation").expect("Should have been able to create the file");
    parse(&table_dict, &mut calgary_tokens, &output_file, "src/test_out/arith.dot")
}

#[test]
fn test_arith_parse() {
    let source_file_content = read_source_file("src/syntax_tests/src/arith.src".to_string());
    let mut scanner = Scanner::from(source_file_content);
    let all_tokens = scanner.get_all_tokens();
    let mut calgary_tokens: Vec<String> = all_tokens.into_iter()
        .map(|token| get_calgary_token(token.token_type)).collect();
    let table_dict = get_table_dict("src/grammars/arith_table.json");
    let output_file = File::create("src/syntax_tests/out/arith.derivation").expect("Should have been able to create the file");
    parse(&table_dict, &mut calgary_tokens, &output_file, "src/test_out/arith.dot")
}


#[test]
fn test_localvar_parse() {
    let source_file_content = read_source_file("src/syntax_tests/src/localvardecl.src".to_string());
    let mut scanner = Scanner::from(source_file_content);
    let all_tokens = scanner.get_all_tokens();
    let mut calgary_tokens: Vec<String> = all_tokens.into_iter()
        .map(|token| get_calgary_token(token.token_type)).collect();
    let table_dict = get_table_dict("src/grammars/localvardecl_table.json");
    let output_file = File::create("src/syntax_tests/out/localvardecl.derivation")
        .expect("Should have been able to create the file");
    parse(&table_dict, &mut calgary_tokens, &output_file, "src/test_out/localvardecl.dot")
}


#[test]
fn test_localvar_parse_harder() {
    let source_file_content = read_source_file("src/syntax_tests/src/localvardecl2.src".to_string());
    let mut scanner = Scanner::from(source_file_content);
    let all_tokens = scanner.get_all_tokens();
    let mut calgary_tokens: Vec<String> = all_tokens.into_iter()
        .map(|token| get_calgary_token(token.token_type)).collect();
    let table_dict = get_table_dict("src/grammars/localvardecl_table.json");
    let output_file = File::create("src/syntax_tests/out/localvardecl2.derivation")
        .expect("Should have been able to create the file");
    parse(&table_dict, &mut calgary_tokens, &output_file, "src/test_out/localvardecl2.dot")
}

#[test]
fn test_generate_graph_file() {
    let edges = Edges(
        vec!(
            ("x".to_string(), "y".to_string()),
            ("y".to_string(), "z".to_string()),
            ("z".to_string(), "exit".to_string()),
            ("exit".to_string(), "end".to_string())
        )
    );
    let mut file = File::create("src/test_out/test_dot_library.dot")
        .expect("Unable to create graph file");
    render_to(&mut file, edges)
}

#[test]
fn test_is_uppercase() {
    assert!("LOVE123".chars().all(is_uppercase_or_number))
}

#[test]
fn test_transfer_ownership() {
    let mut some_string = String::from("A string");
    some_string = modifies_and_returns(some_string);
    assert_eq!(some_string.as_str(), "A string hello");
}

#[test]
fn test_writing_to_a_file() {
    let data_to_write = "This is some file data";
    let mut file = File::create("tmp.txt").expect("Should have been able to create the file");
    foo_write(&file);
    file.write_all(data_to_write.as_bytes()).expect("Unable to write in foo")
}

fn foo_write(mut file: &File) {
    let data_to_write = "foo";
    bar_write(file);
    file.write_all(data_to_write.as_bytes()).expect("Unable to write in foo")
}

fn bar_write(mut file: &File) {
    let data_to_write = "bar";
    file.write_all(data_to_write.as_bytes()).expect("Unable to write in bar")
}


#[test]
pub fn test_hash_map() {
    let mut some_hash_map = HashMap::new();
    some_hash_map.insert(String::from("hello"), String::from("yellow"));
}


#[test]
pub fn test_convert_token_to_calgary() {
    let mut scanner = Scanner::from(String::from("x+y+z+lambda"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<&str> = vec![
        "id",
        "plus",
        "id",
        "plus",
        "id",
        "plus",
        "id",
    ];
    let truth: Vec<String> = truth.into_iter().map(|s| s.to_string()).collect();
    let calgary_tokens: Vec<String> = all_tokens.into_iter()
        .map(|token| get_calgary_token(token.token_type)).collect();
    assert_eq!(calgary_tokens.len(), 7);
    assert_eq!(truth, calgary_tokens);
}
