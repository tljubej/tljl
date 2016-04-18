use super::{IDENTIFIER_RX, DOUBLE_LITERAL_RX, STRING_LITERAL_RX, BOOLEAN_LITERAL_RX,
            OPEN_BRACKETS_RX, CLOSED_BRACKETS_RX, BINARY_NOT_OPERATOR_RX,
            OPEN_PARENTHESIS_RX, CLOSED_PARENTHESIS_RX, OPEN_CURLY_BRACES_RX,
            CLOSED_CURLY_BRACES_RX, ASSIGNMENT_OPERATOR_RX, EQUALITY_OPERATOR_RX,
            ADDITION_OPERATOR_RX, SUBTRACTION_OPERATOR_RX, MULTIPLICATION_OPERATOR_RX,
            DIVISION_OPERATOR_RX, BINARY_OR_OPERATOR_RX, BINARY_AND_OPERATOR_RX,
            KEYWORD_LET_RX, KEYWORD_FN_RX, KEYWORD_STRUCT_RX, COMMA_SEPARATOR_RX, tokenize_str,
            Token};

#[test]
fn identifier_regex_valid() {
    assert!(IDENTIFIER_RX.is_match("foo"));
    assert!(IDENTIFIER_RX.is_match("foo1"));
    assert!(IDENTIFIER_RX.is_match("bar22"));
    assert!(IDENTIFIER_RX.is_match("bar_22"));
    assert!(IDENTIFIER_RX.is_match("bar_22_foo"));
    assert!(IDENTIFIER_RX.is_match("bar_22_foo_1"));
    assert!(IDENTIFIER_RX.is_match("bar_22_foo_1_"));
}

#[test]
fn identifier_regex_invalid() {
    assert!(!IDENTIFIER_RX.is_match("1foo"));
    assert!(!IDENTIFIER_RX.is_match("_1foo"));
    assert!(!IDENTIFIER_RX.is_match("_foo"));
    assert!(!IDENTIFIER_RX.is_match("22foo"));
    assert!(!IDENTIFIER_RX.is_match("_22foo"));
    assert!(!IDENTIFIER_RX.is_match("22_foo"));
}

#[test]
fn double_literal_regex_valid() {
    assert!(DOUBLE_LITERAL_RX.is_match("1"));
    assert!(DOUBLE_LITERAL_RX.is_match(".1"));
    assert!(DOUBLE_LITERAL_RX.is_match("1."));
    assert!(DOUBLE_LITERAL_RX.is_match("12"));
    assert!(DOUBLE_LITERAL_RX.is_match("12.1"));
    assert!(DOUBLE_LITERAL_RX.is_match("12.112"));
    assert!(DOUBLE_LITERAL_RX.is_match("12.112e5"));
}

#[test]
fn string_literal_regex_valid() {
    assert!(STRING_LITERAL_RX.is_match("\"I am a string literal\""));
    assert!(STRING_LITERAL_RX.is_match("\"I am a string \n\n\n\n \t\t literal with newlines\""));
}

#[test]
fn string_literal_regex_invalid() {
    assert!(!STRING_LITERAL_RX.is_match("\" I am not a string literal"));
    assert!(!STRING_LITERAL_RX.is_match("I am not a string literal\""));
    assert!(!STRING_LITERAL_RX.is_match("I am not a string literal"));
}

#[test]
fn boolean_literal_regex_valid() {
    assert!(BOOLEAN_LITERAL_RX.is_match("true"));
    assert!(BOOLEAN_LITERAL_RX.is_match("false"));
}

#[test]
fn boolean_literal_regex_invalid() {
    assert!(!BOOLEAN_LITERAL_RX.is_match("brue"));
    assert!(!BOOLEAN_LITERAL_RX.is_match("bhalse"));
}

#[test]
fn double_literal_regex_invalid() {
    assert!(!DOUBLE_LITERAL_RX.is_match("dsadsd"));
    assert!(!DOUBLE_LITERAL_RX.is_match("h1"));
    assert!(!DOUBLE_LITERAL_RX.is_match("h1."));
    assert!(!DOUBLE_LITERAL_RX.is_match("hg1."));
    assert!(!DOUBLE_LITERAL_RX.is_match("hg1"));
}

#[test]
fn braces_brackets_parenthesis_regex_valid() {
    assert!(OPEN_BRACKETS_RX.is_match("["));
    assert!(OPEN_PARENTHESIS_RX.is_match("("));
    assert!(OPEN_CURLY_BRACES_RX.is_match("{"));
    assert!(CLOSED_BRACKETS_RX.is_match("]"));
    assert!(CLOSED_PARENTHESIS_RX.is_match(")"));
    assert!(CLOSED_CURLY_BRACES_RX.is_match("}"));
}

#[test]
fn brackes_brackets_parenthesis_regex_invalid() {
    assert!(!OPEN_BRACKETS_RX.is_match("}"));
    assert!(!OPEN_PARENTHESIS_RX.is_match("]"));
    assert!(!OPEN_CURLY_BRACES_RX.is_match(")"));
    assert!(!CLOSED_BRACKETS_RX.is_match("{"));
    assert!(!CLOSED_PARENTHESIS_RX.is_match("["));
    assert!(!CLOSED_CURLY_BRACES_RX.is_match("("));
}

#[test]
fn operator_regex_valid() {
    assert!(ASSIGNMENT_OPERATOR_RX.is_match("="));
    assert!(ADDITION_OPERATOR_RX.is_match("+"));
    assert!(SUBTRACTION_OPERATOR_RX.is_match("-"));
    assert!(MULTIPLICATION_OPERATOR_RX.is_match("*"));
    assert!(DIVISION_OPERATOR_RX.is_match("/"));
    assert!(BINARY_OR_OPERATOR_RX.is_match("||"));
    assert!(BINARY_AND_OPERATOR_RX.is_match("&&"));
    assert!(EQUALITY_OPERATOR_RX.is_match("=="));
    assert!(BINARY_NOT_OPERATOR_RX.is_match("!"));
}

#[test]
fn operator_regex_invalid() {
    assert!(!ASSIGNMENT_OPERATOR_RX.is_match("-"));
    assert!(!ASSIGNMENT_OPERATOR_RX.is_match("+"));
    assert!(!ADDITION_OPERATOR_RX.is_match("="));
    assert!(!SUBTRACTION_OPERATOR_RX.is_match("*"));
    assert!(!MULTIPLICATION_OPERATOR_RX.is_match("||"));
    assert!(!DIVISION_OPERATOR_RX.is_match("-"));
    assert!(!BINARY_OR_OPERATOR_RX.is_match("&&"));
    assert!(!BINARY_AND_OPERATOR_RX.is_match("=="));
    assert!(!EQUALITY_OPERATOR_RX.is_match("+"));
    assert!(!BINARY_NOT_OPERATOR_RX.is_match("+"));
}

#[test]
fn keyword_regex_valid() {
    assert!(KEYWORD_LET_RX.is_match("let"));
    assert!(KEYWORD_FN_RX.is_match("fn"));
    assert!(KEYWORD_STRUCT_RX.is_match("struct"));
    assert!(COMMA_SEPARATOR_RX.is_match(","));
}

#[test]
fn keyword_regex_invalid() {
    assert!(!KEYWORD_LET_RX.is_match("lasdasdet"));
    assert!(!KEYWORD_FN_RX.is_match("fdsnsdasd"));
    assert!(!KEYWORD_LET_RX.is_match("fn"));
    assert!(!KEYWORD_FN_RX.is_match("let"));
    assert!(!KEYWORD_STRUCT_RX.is_match("fn"));
    assert!(!COMMA_SEPARATOR_RX.is_match("&&"));
}

#[test]
fn tokenizer_valid() {
    assert_eq!(tokenize_str("let a = fn () {}").expect("Tokenization failed"),
    vec![Token::KeywordLet,
         Token::Identifier("a".to_string()),
         Token::AssignmentOperator,
         Token::KeywordFn,
         Token::OpenParenthesis,
         Token::ClosedParenthesis,
         Token::OpenCurlyBraces,
         Token::ClosedCurlyBraces]);

     assert_eq!(tokenize_str("let a = fn foo(bar, bob) {} \"Literally a string true false struct 3.2\"  true false").expect("Tokenization failed"),
     vec![Token::KeywordLet,
          Token::Identifier("a".to_string()),
          Token::AssignmentOperator,
          Token::KeywordFn,
          Token::Identifier("foo".to_string()),
          Token::OpenParenthesis,
          Token::Identifier("bar".to_string()),
          Token::CommaSeparator,
          Token::Identifier("bob".to_string()),
          Token::ClosedParenthesis,
          Token::OpenCurlyBraces,
          Token::ClosedCurlyBraces,
          Token::StringLiteral("Literally a string true false struct 3.2".to_string()),
          Token::BooleanLiteral("true".to_string()),
          Token::BooleanLiteral("false".to_string())]);

     assert_eq!(tokenize_str("   let a = fn \n\n\n \t\t\t\t () { b = 8.3 }  ").expect("Tokenization failed"),
     vec![Token::KeywordLet,
          Token::Identifier("a".to_string()),
          Token::AssignmentOperator,
          Token::KeywordFn,
          Token::OpenParenthesis,
          Token::ClosedParenthesis,
          Token::OpenCurlyBraces,
          Token::Identifier("b".to_string()),
          Token::AssignmentOperator,
          Token::DoubleLiteral("8.3".to_string()),
          Token::ClosedCurlyBraces]);

      assert_eq!(tokenize_str("&& || === * - /").expect("Tokenization failed"),
      vec![Token::BinaryAndOperator,
           Token::BinaryOrOperator,
           Token::EqualityOperator,
           Token::AssignmentOperator,
           Token::MultiplicationOperator,
           Token::SubtractionOperator,
           Token::DivisionOperator]);
}

#[test]
fn tokenizer_invalid() {
    match tokenize_str("let a = fn () & {}") {
        Ok(_) => { panic!("Example should not sucessfuly tokenize!") }
        Err(_) => assert!(true)
    }

    match tokenize_str("===?") {
        Ok(_) => { panic!("Example should not sucessfuly tokenize!") }
        Err(_) => assert!(true)
    }
}
