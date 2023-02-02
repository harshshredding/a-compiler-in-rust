use std::fs;


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