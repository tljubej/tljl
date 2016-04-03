use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordLet,
    KeywordFn,
    Identifier(String),
    DoubleLiteral(String),
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
    BinaryAndOperator
}

lazy_static!{
    static ref KEYWORD_LET_RX: Regex = Regex::new(r"^let").unwrap();
    static ref KEYWORD_FN_RX: Regex = Regex::new(r"^fn").unwrap();
    static ref IDENTIFIER_RX: Regex = Regex::new(r"^[:alpha:][:alphanum:]*").unwrap();
    static ref DOUBLE_LITERAL_RX: Regex =
        Regex::new(r"^(([1-9][0-9]*\.?[0-9]*)|(\.[0-9]+))([Ee][+-]?[0-9]+)?").unwrap();
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
    static ref ADDITION_OPERATOR_RX: Regex = Regex::new(r"^\+").unwrap();
    static ref SUBTRACTION_OPERATOR_RX: Regex = Regex::new(r"^-").unwrap();
    static ref MULTIPLICATION_OPERATOR_RX: Regex = Regex::new(r"^\*").unwrap();
    static ref DIVISION_OPERATOR_RX: Regex = Regex::new(r"^/").unwrap();
}

macro_rules! tokenize {
    ( $string:expr ,$container:expr, $( $regex:ident | $token_creator:expr ), *) => {
        $string = $string.trim();
        while $string.len() > 0
        {
            $(
                match $regex.find($string) {
                    Some((0, n)) => {
                        $container.push($token_creator((&$string[0..n]).to_string()));
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
            IDENTIFIER_RX | |input| Token::Identifier(input),
            DOUBLE_LITERAL_RX | |input| Token::DoubleLiteral(input),
            OPEN_PARENTHESIS_RX | |_| Token::OpenParenthesis,
            CLOSED_PARENTHESIS_RX | |_| Token::ClosedParenthesis,
            OPEN_BRACKETS_RX | |_| Token::OpenBrackets,
            CLOSED_BRACKETS_RX | |_| Token::ClosedBrackets,
            OPEN_CURLY_BRACES_RX | |_| Token::OpenCurlyBraces,
            CLOSED_CURLY_BRACES_RX | |_| Token::ClosedCurlyBraces,
            EQUALITY_OPERATOR_RX | |_| Token::EqualityOperator,
            BINARY_OR_OPERATOR_RX | |_| Token::BinaryOrOperator,
            BINARY_AND_OPERATOR_RX | |_| Token::BinaryAndOperator,
            ASSIGNMENT_OPERATOR_RX | |_| Token::AssignmentOperator,
            ADDITION_OPERATOR_RX | |_| Token::AdditionOperator,
            SUBTRACTION_OPERATOR_RX | |_| Token::SubtractionOperator,
            MULTIPLICATION_OPERATOR_RX | |_| Token::MultiplicationOperator,
            DIVISION_OPERATOR_RX | |_| Token::DivisionOperator
            );
    Ok(token_container)
}

#[cfg(test)]
mod tests;
