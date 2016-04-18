use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordLet,
    KeywordFn,
    KeywordStruct,
    StringLiteral(String),
    DoubleLiteral(String),
    BooleanLiteral(String),
    Identifier(String),
    OpenParenthesis,
    ClosedParenthesis,
    OpenCurlyBraces,
    ClosedCurlyBraces,
    OpenBrackets,
    ClosedBrackets,
    AssignmentOperator,
    EqualityOperator,
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    BinaryOrOperator,
    BinaryAndOperator,
    BinaryNotOperator,
    CommaSeparator
}

lazy_static!{
    static ref KEYWORD_LET_RX: Regex = Regex::new(r"^let").unwrap();
    static ref KEYWORD_FN_RX: Regex = Regex::new(r"^fn").unwrap();
    static ref KEYWORD_STRUCT_RX: Regex = Regex::new(r"^struct").unwrap();
    static ref IDENTIFIER_RX: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*").unwrap();
    static ref DOUBLE_LITERAL_RX: Regex =
        Regex::new(r"^(([1-9][0-9]*\.?[0-9]*)|(\.[0-9]+))([Ee][+-]?[0-9]+)?").unwrap();
    static ref STRING_LITERAL_RX: Regex = Regex::new("^\"(?s).*?\"").unwrap();
    static ref BOOLEAN_LITERAL_RX: Regex = Regex::new(r"^(true|false)").unwrap();
    static ref OPEN_PARENTHESIS_RX: Regex = Regex::new(r"^\(").unwrap();
    static ref CLOSED_PARENTHESIS_RX: Regex = Regex::new(r"^\)").unwrap();
    static ref OPEN_CURLY_BRACES_RX: Regex = Regex::new(r"^\{").unwrap();
    static ref CLOSED_CURLY_BRACES_RX: Regex = Regex::new(r"^\}").unwrap();
    static ref OPEN_BRACKETS_RX: Regex = Regex::new(r"^\[").unwrap();
    static ref CLOSED_BRACKETS_RX: Regex = Regex::new(r"^\]").unwrap();
    static ref ASSIGNMENT_OPERATOR_RX: Regex = Regex::new(r"^=").unwrap();
    static ref EQUALITY_OPERATOR_RX: Regex = Regex::new(r"^==").unwrap();
    static ref BINARY_AND_OPERATOR_RX: Regex = Regex::new(r"^&&").unwrap();
    static ref BINARY_OR_OPERATOR_RX: Regex = Regex::new(r"^\|\|").unwrap();
    static ref BINARY_NOT_OPERATOR_RX: Regex = Regex::new(r"^!").unwrap();
    static ref ADDITION_OPERATOR_RX: Regex = Regex::new(r"^\+").unwrap();
    static ref SUBTRACTION_OPERATOR_RX: Regex = Regex::new(r"^-").unwrap();
    static ref MULTIPLICATION_OPERATOR_RX: Regex = Regex::new(r"^\*").unwrap();
    static ref DIVISION_OPERATOR_RX: Regex = Regex::new(r"^/").unwrap();
    static ref COMMA_SEPARATOR_RX: Regex = Regex::new(r"^,").unwrap();
}

macro_rules! tokenize {
    ( $string:expr ,$container:expr, $( $regex:ident | $token_creator:expr ), *) => {
        $string = $string.trim();
        while $string.len() > 0
        {
            $(
                match $regex.find($string) {
                    Some((0, n)) => {
                        $container.push($token_creator((&$string[0..n])));
                        $string = &$string[n..].trim();
                        continue;
                    },
                    _ => {}
                }
            )*
            return Err("Cannot tokenize".to_string());
        }
    };
}

fn tokenize_str(mut input: &str) -> Result<Vec<Token>, String> {
    let mut token_container: Vec<Token> = Vec::new();

    tokenize!(input, token_container,
            KEYWORD_LET_RX | |_| Token::KeywordLet,
            KEYWORD_FN_RX | |_| Token::KeywordFn,
            KEYWORD_STRUCT_RX | |_| Token::KeywordStruct,
            STRING_LITERAL_RX |
                |input: &str| Token::StringLiteral(input[1..input.len()-1].to_string()),
            BOOLEAN_LITERAL_RX | |input: &str| Token::BooleanLiteral(input.to_string()),
            IDENTIFIER_RX | |input: &str| Token::Identifier(input.to_string()),
            DOUBLE_LITERAL_RX | |input: &str| Token::DoubleLiteral(input.to_string()),
            OPEN_PARENTHESIS_RX | |_| Token::OpenParenthesis,
            CLOSED_PARENTHESIS_RX | |_| Token::ClosedParenthesis,
            OPEN_BRACKETS_RX | |_| Token::OpenBrackets,
            CLOSED_BRACKETS_RX | |_| Token::ClosedBrackets,
            OPEN_CURLY_BRACES_RX | |_| Token::OpenCurlyBraces,
            CLOSED_CURLY_BRACES_RX | |_| Token::ClosedCurlyBraces,
            EQUALITY_OPERATOR_RX | |_| Token::EqualityOperator,
            BINARY_OR_OPERATOR_RX | |_| Token::BinaryOrOperator,
            BINARY_AND_OPERATOR_RX | |_| Token::BinaryAndOperator,
            BINARY_NOT_OPERATOR_RX | |_| Token::BinaryNotOperator,
            ASSIGNMENT_OPERATOR_RX | |_| Token::AssignmentOperator,
            ADDITION_OPERATOR_RX | |_| Token::AdditionOperator,
            SUBTRACTION_OPERATOR_RX | |_| Token::SubtractionOperator,
            MULTIPLICATION_OPERATOR_RX | |_| Token::MultiplicationOperator,
            DIVISION_OPERATOR_RX | |_| Token::DivisionOperator,
            COMMA_SEPARATOR_RX | |_| Token::CommaSeparator
            );
    Ok(token_container)
}

#[cfg(test)]
mod tests;
