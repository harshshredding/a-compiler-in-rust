use regex::{Captures, Regex};
use std::fs;

// ,, +, -, or, [, intLit, ], =, class, id, {, }, ;, (, ), floatLit, not, :, void, ., *, /, and,
// isa, eq, geq, gt, leq, lt, neq, if, then, else, read, return, while, write, float, integer,
// private, public, function, arrow, constructor, attribute, sr, localVar
#[derive(PartialEq, Eq)]
pub enum TokenType {
    Comma, // ,
    Plus, // +
    Minus, // -
    Or, // or
    OpenSquareBracket, // [
    IntLit, // intLit
    CloseSquareBracket, // ]
    EqualsSymbol, // =
    Class, // class
    Identifier, // id
    OpenCurly, // {
    CloseCurly, // }
    SemiColon, // ;
    OpenParenthesis, // (
    CloseParenthesis, // )
    FloatLit, // floatLit
    Not, // not
    Colon, // :
    Void, // void
    Period, // period
    Asterix, // *
    ForwardSlash, // /
    And, // and
    IsA, // isa
    Eq, // eq
    GreaterThanOrEq, // geq
    Greater, // gt
    LessThanOrEq, // leq
    LessThan, // lt
    NotEqual, // neq
    If, // if
    Then, // then
    Else, // else
    Read, // read
    Return, // return
    While, // while
    Write, // write
    Float, // float
    Integer, // integer
    Private, // private
    Public, // public
    Function, // function
    Arrow, // arrow
    Constructor, // constructor
    Attribute, // attribute
    Sr, // sr
    LocalVar, // localVar
}

pub struct Token {
    token_type: TokenType,
    lexeme: String
}

pub struct Scanner {
    source_text: String,
    current_location: usize,
}

impl Scanner {
    pub fn from(input_source: String) -> Self {
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
            let concerned_slice:String = (&self.source_text[self.current_location..]).into();
            let token_candidates = vec![
                get_float_string(concerned_slice.clone()),
                get_identifier_string(concerned_slice.clone()),
                get_operator_string(concerned_slice.clone()),
                get_whitespaces_string(concerned_slice.clone()),
                get_integer_string(concerned_slice.clone()),
                get_punctuation_string(concerned_slice.clone()),
                get_reserved_keyword_string(concerned_slice.clone()),
            ];
            let token_candidates: Vec<Option<String>> = token_candidates.into_iter().filter(|x| x.is_some()).collect();
            if token_candidates.is_empty() {
                panic!("Could not tokenize !!!");
            }
            let mut longest_string:String = String::from("");
            for candidate in token_candidates {
                let candidate_string = candidate.unwrap();
                if candidate_string.len() > longest_string.len() {
                    longest_string = candidate_string;
                }
            }
            self.current_location += longest_string.len();
            return Some(longest_string); 
        }
    }

    pub fn get_all_tokens(&mut self) -> Vec<String> {
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


fn get_token_if_valid(lexeme_string: Option<String>, token_type: TokenType) -> Option<Token> {
    if lexeme_string.is_some() {
        return Some(Token {token_type: token_type, lexeme: lexeme_string.unwrap() })
    } else {
        return None;
    }
}

#[allow(dead_code)]
pub fn get_identifier_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_identifier_string(source_code_string), TokenType::Identifier);
}

#[allow(dead_code)]
pub fn get_identifier_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^([A-Za-z]\w*)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string)
}


#[allow(dead_code)]
pub fn get_integer_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_integer_string(source_code_string), TokenType::Integer);
}

#[allow(dead_code)]
pub fn get_integer_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(([1-9]\d*)|0)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string)
}

#[allow(dead_code)]
pub fn get_float_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_integer_string(source_code_string), TokenType::Float);
}

#[allow(dead_code)]
pub fn get_float_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(((([1-9]\d*)|0)\.((\d+[1-9])|0))(e[+-](([1-9]\d*)|0))?)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string)
}

#[allow(dead_code)]
pub fn get_operator_token(source_code_string: String) -> Option<Token> {
    let operator_token_string = get_operator_string(source_code_string);
    if operator_token_string.is_none() {
        return None
    } else {
        let token_string = operator_token_string.unwrap();
        match &token_string {
            "==" => return Some(Token { token_type: TokenType::Eq, lexeme: token_string.clone()})
        }
    }
}

#[allow(dead_code)]
pub fn get_operator_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(==|<>|<=|>=|\+|-|\*|/|=|and|or|not)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}


#[allow(dead_code)]
pub fn get_punctuation_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(::|=>|\(|\)|\{|\}|\[|\]|;|,|\.|:)";
    return get_token_using_regex(regex_string.into(), source_code_string)
}

#[allow(dead_code)]
pub fn get_reserved_keyword_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(integer|float|void|class|self|isa|while|if|then|else|read|write|return|localvar|constructor|attribute|function|public|private)([^a-z]|$)";
    return get_token_using_regex(regex_string.into(), source_code_string)
}

#[allow(dead_code)]
pub fn get_whitespaces_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(\s+)(\S|$)";
    return get_token_using_regex(regex_string.into(), source_code_string)
}

fn get_token_using_regex(regex: String, source_code_string: String) -> Option<String> {
    let compiled_regex_obj = Regex::new(&regex).unwrap();
    let captures = compiled_regex_obj.captures(&source_code_string);
    return get_string_from_captures(captures);
}

fn get_string_from_captures(captures_option: Option<Captures>) -> Option<String> {
    match captures_option {
        Some(captures) => {
            let token_string = captures.get(1).unwrap().as_str();
            assert!(!token_string.is_empty());
            return Some(token_string.into());
        }
        None => return None,
    }
}

#[allow(dead_code)]
fn read_source_test_file() -> String {
    let file_path = "src/test_resources/test_code.txt".to_string();
    let file_content = fs::read_to_string(file_path).expect("Could not read contents of file");
    return file_content;
}

#[allow(dead_code)]
fn split_string_by_whitespace(some_string: String) -> Vec<String> {
    let split_up_string = some_string.split(" ").map(|s| s.to_string()).collect();
    return split_up_string;
}

#[allow(dead_code)]
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
        lexeme: String::from("+"),
        token_type: TokenType::Plus
    };
    assert!(some_token.token_type == TokenType::Plus);
    assert_eq!(
        some_token.lexeme,
        String::from("+")
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

    assert!(identifier_regex
        .captures(string_doesnt_start_with_id.as_str())
        .is_none());

    let identifier_capture = identifier_regex
        .captures(&string_starts_with_id)
        .expect("should be able to capture identifier");
    let whole_capture = identifier_capture.get(0).unwrap().as_str();
    assert_eq!(whole_capture, "var1+");

    // let whole_capture = captures.get(0).map_or("", |m| m.as_str());
    // assert_eq!(whole_capture, "");
}
