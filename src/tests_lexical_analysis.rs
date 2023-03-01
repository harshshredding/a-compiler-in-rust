#[cfg(test)]
use std::vec;
use regex::{Captures, Regex};
use super::lexical_analysis::*;

#[test]
fn some_test() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_read_source_file() {
    let file_content = read_source_file(
        String::from("src/test_resources/test_code.txt"));
    assert_eq!(file_content, "x = 2 + 3;")
}

#[test]
fn test_splitting_string() {
    let file_content = read_source_file(
        String::from("src/test_resources/test_code.txt"));
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
        token_type: TokenType::Plus,
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
    assert_eq!(token.lexeme, String::from("x"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token.lexeme, String::from("="));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token.lexeme, String::from("2"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token.lexeme, String::from("+"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token.lexeme, String::from("3"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token.lexeme, String::from("+"));
    let token = scanner.next_token().expect("A token should exist");
    assert_eq!(token.lexeme, String::from("4"));
    assert!(scanner.next_token().is_none())
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

#[test]
fn test_get_integer() {
    let id = get_integer_string(String::from("x+y+z"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("some_id = another_token"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("some_id_123_-another_token"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("+some_token"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("123+some_token"));
    assert!(id.is_some());
    assert_eq!(String::from("123"), id.unwrap());

    let id = get_integer_string(String::from("123 some_token"));
    assert!(id.is_some());
    assert_eq!(String::from("123"), id.unwrap());

    let id = get_integer_string(String::from("123abc"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("1abc+some_token"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("0123"));
    assert!(id.is_none());

    let id = get_integer_string(String::from("0"));
    assert!(id.is_some());
    assert_eq!(String::from("0"), id.unwrap());
}

#[test]
fn test_get_identifier() {
    let id = get_identifier_string(String::from("x+y+z"));
    assert!(id.is_some());
    assert_eq!(String::from("x"), id.unwrap());

    let id = get_identifier_string(String::from("some_id = another_token"));
    assert!(id.is_some());
    assert_eq!(String::from("some_id"), id.unwrap());

    let id = get_identifier_string(String::from("some_id_123_-another_token"));
    assert!(id.is_some());
    assert_eq!(String::from("some_id_123_"), id.unwrap());

    let id = get_identifier_string(String::from("+some_token"));
    assert!(id.is_none());

    let id = get_identifier_string(String::from("123+some_token"));
    assert!(id.is_none());

    let id = get_identifier_string(String::from("some_id"));
    assert!(id.is_some());
    assert_eq!(String::from("some_id"), id.unwrap());

    let id = get_identifier_string(String::from("1abc+some_token"));
    assert!(id.is_none());
}

#[test]
fn test_get_float() {
    let id = get_float_string(String::from("123.34+56"));
    assert!(id.is_some());
    assert_eq!(String::from("123.34"), id.unwrap());

    let id = get_float_string(String::from("123+56"));
    assert!(id.is_none());

    let id = get_float_string(String::from("0.+56"));
    assert!(id.is_none());

    let id = get_float_string(String::from("0.00"));
    assert!(id.is_none());

    let id = get_float_string(String::from("0.003e+3"));
    assert!(id.is_some());
    assert_eq!(String::from("0.003e+3"), id.unwrap());

    let id = get_float_string(String::from("0.00e+3"));
    assert!(id.is_none());

    let id = get_float_string(String::from("0.0e-323 + 4"));
    assert!(id.is_some());
    assert_eq!(String::from("0.0e-323"), id.unwrap());

    let id = get_float_string(String::from("0.0e-00"));
    assert!(id.is_none());

    let id = get_float_string(String::from("0.0e-0"));
    assert!(id.is_some());
    assert_eq!(String::from("0.0e-0"), id.unwrap());


    let id = get_float_string(String::from("0"));
    assert!(id.is_none());
}

#[test]
fn test_get_operator() {
    let id = get_operator_string(String::from("+56"));
    assert!(id.is_some());
    assert_eq!(String::from("+"), id.unwrap());

    let id = get_operator_string(String::from("-ax"));
    assert!(id.is_some());
    assert_eq!(String::from("-"), id.unwrap());

    let id = get_operator_string(String::from("<=ax"));
    assert!(id.is_some());
    assert_eq!(String::from("<="), id.unwrap());

    let id = get_operator_string(String::from(">=ax"));
    assert!(id.is_some());
    assert_eq!(String::from(">="), id.unwrap());

    let id = get_operator_string(String::from("==ax"));
    assert!(id.is_some());
    assert_eq!(String::from("=="), id.unwrap());

    let id = get_operator_string(String::from("=ax"));
    assert!(id.is_some());
    assert_eq!(String::from("="), id.unwrap());

    let id = get_operator_string(String::from("*b"));
    assert!(id.is_some());
    assert_eq!(String::from("*"), id.unwrap());

    let id = get_operator_string(String::from("and 3"));
    assert!(id.is_some());
    assert_eq!(String::from("and"), id.unwrap());

    let id = get_operator_string(String::from("or3"));
    assert!(id.is_some());
    assert_eq!(String::from("or"), id.unwrap());

    let id = get_operator_string(String::from("or 3"));
    assert!(id.is_some());
    assert_eq!(String::from("or"), id.unwrap());

    let id = get_operator_string(String::from("not x"));
    assert!(id.is_some());
    assert_eq!(String::from("not"), id.unwrap());
}

#[test]
fn test_get_punctuation() {
    let punctuation_string = get_punctuation_string(String::from("not x"));
    assert!(punctuation_string.is_none());


    let punctuation_string = get_punctuation_string(String::from(";not x"));
    assert!(punctuation_string.is_some());
    assert_eq!(String::from(";"), punctuation_string.unwrap());


    let punctuation_string = get_punctuation_string(String::from("( not x )"));
    assert!(punctuation_string.is_some());
    assert_eq!(String::from("("), punctuation_string.unwrap());
}

#[test]
fn test_get_reserved_keyword() {
    let reserved_string = get_reserved_keyword_string(String::from("constructor y;"));
    assert!(reserved_string.is_some());
    assert_eq!(String::from("constructor"), reserved_string.unwrap());


    let reserved_string = get_reserved_keyword_string(String::from("selfa"));
    assert!(reserved_string.is_none());


    let reserved_string = get_reserved_keyword_string(String::from("attribute$"));
    assert!(reserved_string.is_some());
    assert_eq!(String::from("attribute"), reserved_string.unwrap());
}


#[test]
fn test_get_whitespace() {
    let reserved_string = get_whitespaces_string(String::from("selfa"));
    assert!(reserved_string.is_none());


    let reserved_string = get_whitespaces_string(String::from(" self y;"));
    assert!(reserved_string.is_some());
    assert_eq!(String::from(" "), reserved_string.unwrap());


    let reserved_string = get_whitespaces_string(String::from(" \t\t\t\t\t\n\n"));
    assert!(reserved_string.is_some());
    assert_eq!(String::from(" \t\t\t\t\t\n\n"), reserved_string.unwrap());
}

#[test]
fn test_get_all_tokens() {
    let mut scanner = Scanner::from(String::from("s"));
    let all_tokens = scanner.get_all_tokens();
    let truth = vec![Token::from(TokenType::Identifier, String::from("s"))];
    assert_eq!(all_tokens, truth);

    let mut scanner = Scanner::from(String::from("x+y+z+lambda"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![Token::from(TokenType::Identifier, String::from("x")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("y")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("z")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("lambda")),
    ];
    assert_eq!(all_tokens, truth);

    let mut scanner = Scanner::from(String::from("x + y +z+lambda   \n"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![Token::from(TokenType::Identifier, String::from("x")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("y")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("z")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("lambda")),
    ];
    assert_eq!(all_tokens, truth);

    let mut scanner = Scanner::from(String::from("lambda=x + y +z+lambda   \n;lambda = 0;"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![Token::from(TokenType::Identifier, String::from("lambda")),
                                 Token::from(TokenType::EqualsSymbol, String::from("=")),
                                 Token::from(TokenType::Identifier, String::from("x")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("y")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("z")),
                                 Token::from(TokenType::Plus, String::from("+")),
                                 Token::from(TokenType::Identifier, String::from("lambda")),
                                 Token::from(TokenType::SemiColon, String::from(";")),
                                 Token::from(TokenType::Identifier, String::from("lambda")),
                                 Token::from(TokenType::EqualsSymbol, String::from("=")),
                                 Token::from(TokenType::IntLit, String::from("0")),
                                 Token::from(TokenType::SemiColon, String::from(";")),
    ];
    assert_eq!(all_tokens, truth);


    let mut scanner = Scanner::from(String::from("z=0+1+2+3-40;"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![
        Token::from(TokenType::Identifier, String::from("z")),
        Token::from(TokenType::EqualsSymbol, String::from("=")),
        Token::from(TokenType::IntLit, String::from("0")),
        Token::from(TokenType::Plus, String::from("+")),
        Token::from(TokenType::IntLit, String::from("1")),
        Token::from(TokenType::Plus, String::from("+")),
        Token::from(TokenType::IntLit, String::from("2")),
        Token::from(TokenType::Plus, String::from("+")),
        Token::from(TokenType::IntLit, String::from("3")),
        Token::from(TokenType::Minus, String::from("-")),
        Token::from(TokenType::IntLit, String::from("40")),
        Token::from(TokenType::SemiColon, String::from(";")),
    ];
    assert_eq!(all_tokens, truth);

    let mut scanner = Scanner::from(String::from("function bubbleSort(arr: integer[], size: integer) => void"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![
        Token::from(TokenType::Function, String::from("function")),
        Token::from(TokenType::Identifier, String::from("bubbleSort")),
        Token::from(TokenType::OpenParenthesis, String::from("(")),
        Token::from(TokenType::Identifier, String::from("arr")),
        Token::from(TokenType::Colon, String::from(":")),
        Token::from(TokenType::IntegerKeyword, String::from("integer")),
        Token::from(TokenType::OpenSquareBracket, String::from("[")),
        Token::from(TokenType::CloseSquareBracket, String::from("]")),
        Token::from(TokenType::Comma, String::from(",")),
        Token::from(TokenType::Identifier, String::from("size")),
        Token::from(TokenType::Colon, String::from(":")),
        Token::from(TokenType::IntegerKeyword, String::from("integer")),
        Token::from(TokenType::CloseParenthesis, String::from(")")),
        Token::from(TokenType::Arrow, String::from("=>")),
        Token::from(TokenType::Void, String::from("void")),
    ];
    assert_eq!(all_tokens, truth);



    let mut scanner = Scanner::from(String::from("3.5"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![
        Token::from(TokenType::FloatLit, String::from("3.5")),
    ];
    assert_eq!(all_tokens, truth);


    let mut scanner = Scanner::from(String::from("f1: LINEAR(2, 3.5)"));
    let all_tokens = scanner.get_all_tokens();
    let truth: Vec<Token> = vec![
        Token::from(TokenType::Identifier, String::from("f1")),
        Token::from(TokenType::Colon, String::from(":")),
        Token::from(TokenType::Identifier, String::from("LINEAR")),
        Token::from(TokenType::OpenParenthesis, String::from("(")),
        Token::from(TokenType::IntLit, String::from("2")),
        Token::from(TokenType::Comma, String::from(",")),
        Token::from(TokenType::FloatLit, String::from("3.5")),
        Token::from(TokenType::CloseParenthesis, String::from(")")),
    ];
    assert_eq!(all_tokens, truth);
}

// #[test]
// fn test_single_line_comment() {
//     panic!("not implemented")
// }

#[test]
fn test_get_reserved_word_token() {
    let reserved_token = get_reserved_word_token(String::from("read y;"));
    assert!(reserved_token.is_some());
    let reserved_token = reserved_token.unwrap();
    assert_eq!(reserved_token, Token::from(TokenType::Read, String::from("read")));

    // let reserved_string = get_reserved_keyword_string(String::from("selfa"));
    // assert!(reserved_string.is_none());


    // let reserved_string = get_reserved_keyword_string(String::from("self$"));
    // assert!(reserved_string.is_some());
    // assert_eq!(String::from("self"), reserved_string.unwrap());
}