#![allow(dead_code)]

mod lexical_analysis;
mod synthesis;
mod tests_lexical_analysis;
mod semantic_analysis;
mod syntactic_analysis;
mod tests_syntactic_analysis;

use lexical_analysis::read_source_file;
use lexical_analysis::Scanner;
use std::fs::File;
use std::io::prelude::*;
use crate::lexical_analysis::Token;
use std::env;
use regex::{Captures, Regex};


fn write_tokens_to_file(tokens: Vec<Token>, output_file_path: String) -> std::io::Result<()>  {
    let mut output_tokens_file = File::create(output_file_path)?;
    for token in tokens {
        let token_type_string = format!("{:?}\n", token.token_type);
        output_tokens_file.write_all(token_type_string.as_bytes())?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "missing arguments");
    let source_file_path = args[1].clone();
    let output_tokens_file_path = args[2].clone();
    let source_file_content = read_source_file(source_file_path);
    let mut scanner = Scanner::from(source_file_content);
    let all_tokens = scanner.get_all_tokens();
    write_tokens_to_file(all_tokens, output_tokens_file_path.clone())?;
    println!("successfully generated {}", output_tokens_file_path.as_str());
    Ok(())
}


// #[test]
// fn test_regex() {
//     println!("hello world!");
// }
//
// fn test_regex() {
//     let compiled_regex_obj = Regex::new("x").unwrap();
// }