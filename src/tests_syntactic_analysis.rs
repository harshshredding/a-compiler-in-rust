use super::lexical_analysis::Scanner;
use std::collections::HashMap;
use super::syntactic_analysis::get_calgary_token;
use std::fs::File;
use std::io::Write;

#[cfg(test)]

fn modifies_and_returns(mut some_string: String) -> String {
    some_string = some_string + " hello";
    return some_string
}

#[test]
fn test_split_a_string() {
    let string = String::from("a → X + Y");
    let split_string: Vec<&str> = string.split_whitespace().collect();
    assert_eq!(split_string, vec!["a","→","X","+","Y"])
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
        "id"
    ];
    let truth: Vec<String> = truth.into_iter().map(|s| s.to_string()).collect();
    let calgary_tokens: Vec<String> = all_tokens.into_iter()
        .map(|token| get_calgary_token(token.token_type)).collect();
    assert_eq!(calgary_tokens.len(), 7);
    assert_eq!(truth, calgary_tokens);
}
