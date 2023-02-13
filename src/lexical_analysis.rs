use regex::Regex;
use std::fs;

// paranthesis: [, ], {, }, (, )
// operator: +, -, *, /, =, ==, <>, <, >, <=, >=
// split: (split_atom)*
// split_atom: operator | paranthesis | space
// identifier: alpha
// Regex: (identifier split)*
// - token*

enum TokenType {
    Split,
    Identifier,
}

struct Token {
    token_text: String,
}

struct Scanner {
    source_text: String,
    current_location: usize,
}

impl Scanner {
    fn from(input_source: String) -> Self {
        return Self {
            source_text: input_source,
            current_location: 0,
        };
    }

    fn source_size(&self) -> usize {
        return self.source_text.len();
    }

    fn next_token(&mut self) -> Option<String> {
        if self.current_location > self.source_size() {
            panic!(
                "current location {} is greater than source len {}",
                self.current_location,
                self.source_text.len()
            );
        } else if self.current_location == self.source_size() {
            return None;
        } else {
            let concerned_slice = &self.source_text[self.current_location..];
            for (i, char) in concerned_slice.chars().enumerate() {
                if !char.is_alphanumeric() {
                    let token_string = &concerned_slice[..i];
                    if token_string.is_empty() {
                        self.current_location += 1;
                        return Some(char.to_string());
                    } else {
                        return Some(token_string.to_string());
                    }
                } else {
                    self.current_location += 1;
                }
            }
            if !concerned_slice.is_empty() {
                return Some(concerned_slice.to_string());
            } else {
                return None;
            }
        }
    }

    fn get_all_tokens(&mut self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        loop {
            match self.next_token() {
                Some(token) => {
                    if !token.trim().is_empty() {
                        // ignore whitespaces
                        ret.push(token)
                    }
                }
                None => {
                    return ret;
                }
            }
        }
    }
}

fn get_identifier(source_code_string: String) -> Option<String> {
    // Add new line char to source code
    let source_code_string = source_code_string.clone() + "\n";

    let identifier_regex = Regex::new(r"^([A-Za-z]\w*)[^\w]").unwrap();

    let identifier_capture = identifier_regex.captures(&source_code_string);

    match identifier_capture {
        Some(captures) => {
            let identifier_string = captures.get(1).unwrap().as_str();
            assert!(!identifier_string.is_empty());
            return Some(identifier_string.into());
        },
        None => return None
    }
}

fn get_integer(source_code_string: String) -> Option<String> {
    // Add new line char to source code
    let source_code_string = source_code_string.clone() + "\n";

    let digit_regex = Regex::new(r"^(\d+)[^\w]").unwrap();

    let identifier_capture = digit_regex.captures(&source_code_string);

    match identifier_capture {
        Some(captures) => {
            let integer_string = captures.get(1).unwrap().as_str();
            assert!(!integer_string.is_empty());
            return Some(integer_string.into());
        },
        None => return None
    }
}

fn read_source_test_file() -> String {
    let file_path = "src/test_resources/test_code.txt".to_string();
    let file_content = fs::read_to_string(file_path).expect("Could not read contents of file");
    return file_content;
}

fn split_string_by_whitespace(some_string: String) -> Vec<String> {
    let split_up_string = some_string.split(" ").map(|s| s.to_string()).collect();
    return split_up_string;
}

fn scan() -> String {
    String::from("A token")
}

#[test]
fn some_test() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_read_source_file() {
    let file_content = read_source_test_file();
    assert_eq!(file_content, "x = 2 + 3;")
}

#[test]
fn test_splitting_string() {
    let file_content = read_source_test_file();
    let split_up_string = split_string_by_whitespace(file_content);
    let expected_result = vec![
        String::from("x"),
        String::from("="),
        String::from("2"),
        String::from("+"),
        String::from("3;"),
    ];
    assert_eq!(split_up_string, expected_result);
}

#[test]
fn test_create_token() {
    let some_token = Token {
        token_text: String::from("This is a token string"),
    };
    assert_eq!(
        some_token.token_text,
        String::from("This is a token string")
    );
}

#[test]
fn test_create_scanner() {
    let source = String::from("123");
    let scanner = Scanner {
        source_text: source,
        current_location: 10,
    };
    assert_eq!(scanner.source_text, String::from("123"));
    assert_eq!(scanner.current_location, 10);
}

#[test]
fn test_scanner_create_using_method() {
    let scanner = Scanner::from(String::from("x=y+z"));
    assert_eq!(scanner.source_text, String::from("x=y+z"));
    assert_eq!(scanner.current_location, 0);
}

#[test]
fn test_next_token() {
    let mut scanner = Scanner::from(String::from("x=2+3+4"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("x"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("="));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("2"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("+"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("3"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("+"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token, String::from("4"));
    assert!(scanner.next_token().is_none())
}

#[test]
fn test_get_all_tokens() {
    let mut scanner = Scanner::from(String::from("yz+ j"));
    let all_tokens = scanner.get_all_tokens();
    assert!(all_tokens.len() == 3);
}

#[test]
fn test_with_long_identifiers() {
    let mut scanner = Scanner::from(String::from("hello    from the other   sideeeeeee"));
    let all_tokens = scanner.get_all_tokens();
    assert!(all_tokens.len() == 5);
}

#[test]
fn test_regular_expressions_to_find_first_identifier() {
    let string_doesnt_start_with_id = String::from("123var1+23");
    let string_starts_with_id = String::from("var1+var2");

    let identifier_regex = Regex::new(r"^([A-Za-z]\w*)[^\w]").unwrap();

    assert!(identifier_regex.captures(string_doesnt_start_with_id.as_str()).is_none());

    let identifier_capture = identifier_regex.captures(&string_starts_with_id)
                                                            .expect("should be able to capture identifier");
    let whole_capture = identifier_capture.get(0).unwrap().as_str();
    assert_eq!(whole_capture, "var1+");

    // let whole_capture = captures.get(0).map_or("", |m| m.as_str());
    // assert_eq!(whole_capture, "");
}

#[test]
fn test_get_identifier() {
    let id = get_identifier(String::from("x+y+z"));
    assert!(id.is_some());
    assert_eq!(String::from("x"), id.unwrap());


    let id = get_identifier(String::from("some_id = another_token"));
    assert!(id.is_some());
    assert_eq!(String::from("some_id"), id.unwrap());


    let id = get_identifier(String::from("some_id_123_-another_token"));
    assert!(id.is_some());
    assert_eq!(String::from("some_id_123_"), id.unwrap());


    let id = get_identifier(String::from("+some_token"));
    assert!(id.is_none());

    let id = get_identifier(String::from("123+some_token"));
    assert!(id.is_none());

    
    let id = get_identifier(String::from("some_id"));
    assert!(id.is_some());
    assert_eq!(String::from("some_id"), id.unwrap());

    
    let id = get_identifier(String::from("1abc+some_token"));
    assert!(id.is_none());

}

#[test]
fn test_get_integer() {
    let id = get_integer(String::from("x+y+z"));
    assert!(id.is_none());


    let id = get_integer(String::from("some_id = another_token"));
    assert!(id.is_none());


    let id = get_integer(String::from("some_id_123_-another_token"));
    assert!(id.is_none());


    let id = get_integer(String::from("+some_token"));
    assert!(id.is_none());

    let id = get_integer(String::from("123+some_token"));
    assert!(id.is_some());
    assert_eq!(String::from("123"), id.unwrap());

    let id = get_integer(String::from("123 some_token"));
    assert!(id.is_some());
    assert_eq!(String::from("123"), id.unwrap());

    
    let id = get_integer(String::from("123abc"));
    assert!(id.is_none());

    
    let id = get_integer(String::from("1abc+some_token"));
    assert!(id.is_none());

}


