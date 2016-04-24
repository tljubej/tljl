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
    vec![(0, Token::KeywordLet),
         (4, Token::Identifier("a".to_string())),
         (6, Token::AssignmentOperator),
         (8, Token::KeywordFn),
         (11, Token::OpenParenthesis),
         (12, Token::ClosedParenthesis),
         (14, Token::OpenCurlyBraces),
         (15, Token::ClosedCurlyBraces)]);

     assert_eq!(tokenize_str("let a = fn foo(bar, bob) {} \
         \"Literally a string true false struct 3.2\"  true false").expect("Tokenization failed"),
     vec![(0, Token::KeywordLet),
          (4, Token::Identifier("a".to_string())),
          (6, Token::AssignmentOperator),
          (8, Token::KeywordFn),
          (11, Token::Identifier("foo".to_string())),
          (14, Token::OpenParenthesis),
          (15, Token::Identifier("bar".to_string())),
          (18, Token::CommaSeparator),
          (20, Token::Identifier("bob".to_string())),
          (23, Token::ClosedParenthesis),
          (25, Token::OpenCurlyBraces),
          (26, Token::ClosedCurlyBraces),
          (28, Token::StringLiteral("Literally a string true false struct 3.2".to_string())),
          (72, Token::BooleanLiteral("true".to_string())),
          (77, Token::BooleanLiteral("false".to_string()))]);

     assert_eq!(tokenize_str("   let a = fn \n\n\n \t\t\t\t () { b = 8.3 }  ")
        .expect("Tokenization failed"),
     vec![(3, Token::KeywordLet),
          (7, Token::Identifier("a".to_string())),
          (9, Token::AssignmentOperator),
          (11, Token::KeywordFn),
          (23, Token::OpenParenthesis),
          (24, Token::ClosedParenthesis),
          (26, Token::OpenCurlyBraces),
          (28, Token::Identifier("b".to_string())),
          (30, Token::AssignmentOperator),
          (32, Token::DoubleLiteral("8.3".to_string())),
          (36, Token::ClosedCurlyBraces)]);

      assert_eq!(tokenize_str("&& || === * - /").expect("Tokenization failed"),
      vec![(0, Token::BinaryAndOperator),
           (3, Token::BinaryOrOperator),
           (6, Token::EqualityOperator),
           (8, Token::AssignmentOperator),
           (10, Token::MultiplicationOperator),
           (12, Token::SubtractionOperator),
           (14, Token::DivisionOperator)]);
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
