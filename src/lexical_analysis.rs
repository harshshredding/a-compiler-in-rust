use regex::{Captures, Regex};
use std::fs;

// ,, +, -, or, [, intLit, ], =, class, id, {, }, ;, (, ), floatLit, not, :, void, ., *, /, and,
// isa, eq, geq, gt, leq, lt, neq, if, then, else, read, return, while, write, float, integer,
// private, public, function, arrow, constructor, attribute, sr, localVar
#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Comma,
    // ,
    Plus,
    // +
    Minus,
    // -
    Or,
    // or
    OpenSquareBracket,
    // [
    IntLit,
    // intLit
    CloseSquareBracket,
    // ]
    EqualsSymbol,
    // =
    Class,
    // class
    Identifier,
    // id
    OpenCurly,
    // {
    CloseCurly,
    // }
    SemiColon,
    // ;
    OpenParenthesis,
    // (
    CloseParenthesis,
    // )
    FloatLit,
    // floatLit
    Not,
    // not
    Colon,
    // :
    Void,
    // void
    Period,
    // period
    Asterix,
    // *
    ForwardSlash,
    // /
    And,
    // and
    IsA,
    // isa
    Eq,
    // eq
    GreaterThanOrEq,
    // geq
    GreaterThan,
    // gt
    LessThanOrEq,
    // leq
    LessThan,
    // lt
    NotEqual,
    // neq
    If,
    // if
    Then,
    // then
    Else,
    // else
    Read,
    // read
    Return,
    // return
    While,
    // while
    Write,
    // write
    FloatKeyword,
    // float
    IntegerKeyword,
    // integer
    Private,
    // private
    Public,
    // public
    Function,
    // function
    Arrow,
    // arrow
    Constructor,
    // constructor
    Attribute,
    // attribute
    Sr,
    // sr
    LocalVar,
    // self
    WhiteSpace, // represents whitespaces
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

pub struct Scanner {
    pub source_text: String,
    pub current_location: usize,
}

impl Token {
    pub fn from(token_type: TokenType, lexeme: String) -> Self {
        return Self {
            token_type,
            lexeme,
        };
    }
}

impl Scanner {
    pub fn from(input_source: String) -> Self {
        return Self {
            source_text: input_source,
            current_location: 0,
        };
    }

    pub fn source_size(&self) -> usize {
        return self.source_text.len();
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_location > self.source_size() {
            panic!(
                "current location {} is greater than source len {}",
                self.current_location,
                self.source_text.len()
            );
        } else if self.current_location == self.source_size() {
            return None;
        } else {
            let concerned_slice: String = (&self.source_text[self.current_location..]).into();
            let token_candidates = vec![
                get_float_token(concerned_slice.clone()),
                get_identifier_token(concerned_slice.clone()),
                get_operator_token(concerned_slice.clone()),
                get_whitespace_token(concerned_slice.clone()),
                get_integer_token(concerned_slice.clone()),
                get_punctuation_token(concerned_slice.clone()),
                get_reserved_word_token(concerned_slice.clone()),
            ];
            let token_candidates: Vec<Option<Token>> = token_candidates.into_iter().filter(|x| x.is_some()).collect();
            if token_candidates.is_empty() {
                panic!("Could not tokenize remaining string:\n{}", concerned_slice.as_str());
            }
            let mut longest_token: Option<Token> = None;
            for candidate in token_candidates {
                assert!(candidate.is_some());
                let candidate_token = candidate.unwrap();
                let longest_token_ref = longest_token.as_ref();
                if (longest_token_ref.is_none()) || (candidate_token.lexeme.len() > longest_token_ref.unwrap().lexeme.len()) {
                    longest_token = Some(candidate_token);
                } else if longest_token_ref.is_some() && (longest_token_ref.unwrap().lexeme.len() == candidate_token.lexeme.len()) {
                    if (longest_token_ref.unwrap().token_type == TokenType::Identifier)
                        && (candidate_token.token_type != TokenType::Identifier) {
                        longest_token = Some(candidate_token);
                    }
                }
            }
            self.current_location += longest_token.as_ref().unwrap().lexeme.len();
            return longest_token;
        }
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut ret: Vec<Token> = Vec::new();
        loop {
            match self.next_token() {
                Some(token) => {
                    if token.token_type != TokenType::WhiteSpace {
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
    return if lexeme_string.is_some() {
        Some(Token { token_type: token_type, lexeme: lexeme_string.unwrap() })
    } else {
        None
    }
}

pub fn get_identifier_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_identifier_string(source_code_string), TokenType::Identifier);
}

pub fn get_identifier_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^([A-Za-z]\w*)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}

pub fn get_integer_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_integer_string(source_code_string), TokenType::IntLit);
}

pub fn get_integer_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(([1-9]\d*)|0)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}

pub fn get_float_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_float_string(source_code_string), TokenType::FloatLit);
}

pub fn get_float_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(((([1-9]\d*)|0)\.((\d+[1-9])|0))(e[+-](([1-9]\d*)|0))?)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}

pub fn get_operator_token(source_code_string: String) -> Option<Token> {
    let operator_token_string = get_operator_string(source_code_string);
    if operator_token_string.is_none() {
        return None;
    } else {
        let operator_string = operator_token_string.unwrap();
        match operator_string.as_str() {
            "==" => return Some(Token { token_type: TokenType::Eq, lexeme: operator_string }),
            "<>" => return Some(Token { token_type: TokenType::NotEqual, lexeme: operator_string }),
            "<=" => return Some(Token { token_type: TokenType::LessThanOrEq, lexeme: operator_string }),
            ">=" => return Some(Token { token_type: TokenType::GreaterThanOrEq, lexeme: operator_string }),
            ">" => return Some(Token { token_type: TokenType::GreaterThan, lexeme: operator_string }),
            "<" => return Some(Token { token_type: TokenType::LessThan, lexeme: operator_string }),
            "+" => return Some(Token { token_type: TokenType::Plus, lexeme: operator_string }),
            "-" => return Some(Token { token_type: TokenType::Minus, lexeme: operator_string }),
            "*" => return Some(Token { token_type: TokenType::Asterix, lexeme: operator_string }),
            "/" => return Some(Token { token_type: TokenType::ForwardSlash, lexeme: operator_string }),
            "=" => return Some(Token { token_type: TokenType::EqualsSymbol, lexeme: operator_string }),
            "and" => return Some(Token { token_type: TokenType::And, lexeme: operator_string }),
            "or" => return Some(Token { token_type: TokenType::Or, lexeme: operator_string }),
            "not" => return Some(Token { token_type: TokenType::Not, lexeme: operator_string }),
            _ => panic!("{} is not an operator", operator_string.as_str())
        }
    }
}

pub fn get_operator_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(==|<>|<=|>=|\+|-|\*|/|=|and|or|not|<|>)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}

pub fn get_punctuation_token(source_code_string: String) -> Option<Token> {
    let punctuation_string = get_punctuation_string(source_code_string);
    if punctuation_string.is_none() {
        return None;
    } else {
        let punctuation_string = punctuation_string.unwrap();
        match punctuation_string.as_str() {
            "::" => return Some(Token { token_type: TokenType::Sr, lexeme: punctuation_string }),
            "=>" => return Some(Token { token_type: TokenType::Arrow, lexeme: punctuation_string }),
            "(" => return Some(Token { token_type: TokenType::OpenParenthesis, lexeme: punctuation_string }),
            ")" => return Some(Token { token_type: TokenType::CloseParenthesis, lexeme: punctuation_string }),
            "{" => return Some(Token { token_type: TokenType::OpenCurly, lexeme: punctuation_string }),
            "}" => return Some(Token { token_type: TokenType::CloseCurly, lexeme: punctuation_string }),
            "[" => return Some(Token { token_type: TokenType::OpenSquareBracket, lexeme: punctuation_string }),
            "]" => return Some(Token { token_type: TokenType::CloseSquareBracket, lexeme: punctuation_string }),
            ";" => return Some(Token { token_type: TokenType::SemiColon, lexeme: punctuation_string }),
            "," => return Some(Token { token_type: TokenType::Comma, lexeme: punctuation_string }),
            "." => return Some(Token { token_type: TokenType::Period, lexeme: punctuation_string }),
            ":" => return Some(Token { token_type: TokenType::Colon, lexeme: punctuation_string }),
            _ => panic!("{} is not a punctuation", punctuation_string.as_str())
        }
    }
}

pub fn get_punctuation_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(::|=>|\(|\)|\{|\}|\[|\]|;|,|\.|:)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}


pub fn get_reserved_word_token(source_code_string: String) -> Option<Token> {
    let reserved_keyword_string = get_reserved_keyword_string(source_code_string);
    if reserved_keyword_string.is_none() {
        return None;
    } else {
        let reserved_keyword_string = reserved_keyword_string.unwrap();
        match reserved_keyword_string.as_str() {
            "integer" => return Some(Token { token_type: TokenType::IntegerKeyword, lexeme: reserved_keyword_string }),
            "float" => return Some(Token { token_type: TokenType::FloatKeyword, lexeme: reserved_keyword_string }),
            "void" => return Some(Token { token_type: TokenType::Void, lexeme: reserved_keyword_string }),
            "class" => return Some(Token { token_type: TokenType::Class, lexeme: reserved_keyword_string }),
            "isa" => return Some(Token { token_type: TokenType::IsA, lexeme: reserved_keyword_string }),
            "while" => return Some(Token { token_type: TokenType::While, lexeme: reserved_keyword_string }),
            "if" => return Some(Token { token_type: TokenType::If, lexeme: reserved_keyword_string }),
            "then" => return Some(Token { token_type: TokenType::Then, lexeme: reserved_keyword_string }),
            "else" => return Some(Token { token_type: TokenType::Else, lexeme: reserved_keyword_string }),
            "read" => return Some(Token { token_type: TokenType::Read, lexeme: reserved_keyword_string }),
            "write" => return Some(Token { token_type: TokenType::Write, lexeme: reserved_keyword_string }),
            "return" => return Some(Token { token_type: TokenType::Return, lexeme: reserved_keyword_string }),
            "localvar" => return Some(Token { token_type: TokenType::LocalVar, lexeme: reserved_keyword_string }),
            "constructor" => return Some(Token { token_type: TokenType::Constructor, lexeme: reserved_keyword_string }),
            "attribute" => return Some(Token { token_type: TokenType::Attribute, lexeme: reserved_keyword_string }),
            "function" => return Some(Token { token_type: TokenType::Function, lexeme: reserved_keyword_string }),
            "public" => return Some(Token { token_type: TokenType::Public, lexeme: reserved_keyword_string }),
            "private" => return Some(Token { token_type: TokenType::Private, lexeme: reserved_keyword_string }),
            _ => panic!("{} is not a reserved keyword", reserved_keyword_string.as_str())
        }
    }
}

pub fn get_reserved_keyword_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(integer|float|void|class|isa|while|if|then|else|read|write|return|localvar|constructor|attribute|function|public|private)(\W|$)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}


pub fn get_whitespace_token(source_code_string: String) -> Option<Token> {
    return get_token_if_valid(get_whitespaces_string(source_code_string), TokenType::WhiteSpace);
}

pub fn get_whitespaces_string(source_code_string: String) -> Option<String> {
    let regex_string = r"^(\s+)(\S|$)";
    return get_token_using_regex(regex_string.into(), source_code_string);
}

pub fn get_token_using_regex(regex: String, source_code_string: String) -> Option<String> {
    let compiled_regex_obj = Regex::new(&regex).unwrap();
    let captures = compiled_regex_obj.captures(&source_code_string);
    return get_string_from_captures(captures);
}

pub fn get_string_from_captures(captures_option: Option<Captures>) -> Option<String> {
    return match captures_option {
        Some(captures) => {
            let token_string = captures.get(1).unwrap().as_str();
            assert!(!token_string.is_empty());
            Some(token_string.into())
        }
        None => None,
    }
}

pub fn read_source_file(source_file_path: String) -> String {
    let file_content = fs::read_to_string(source_file_path).expect("Could not read contents of file");
    return file_content;
}

pub fn split_string_by_whitespace(some_string: String) -> Vec<String> {
    let split_up_string = some_string.split(" ").map(|s| s.to_string()).collect();
    return split_up_string;
}

pub fn scan() -> String {
    String::from("A token")
}

