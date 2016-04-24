use regex::Regex;

pub type SourcePosition = usize;
pub struct SourcePositionRowColumn(usize, usize);

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordLet,
    KeywordFn,
    KeywordStruct,
    Keywordwhile,
    KeywordFor,
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
    CommaSeparator,
}

lazy_static!{
    static ref KEYWORD_LET_RX: Regex = Regex::new(r"^let").unwrap();
    static ref KEYWORD_FN_RX: Regex = Regex::new(r"^fn").unwrap();
    static ref KEYWORD_STRUCT_RX: Regex = Regex::new(r"^struct").unwrap();
    static ref KEYWORD_WHILE_RX: Regex = Regex::new(r"^while").unwrap();
    static ref KEYWORD_FOR_RX: Regex = Regex::new(r"^for").unwrap();
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
    ( $string:expr ,$container:expr, $pos_counter:expr,
        $( $regex:ident | $token_creator:expr ), *) => {{

        while $string.len() > 0
        {
            let trimmed_string = $string.trim_left();
            if trimmed_string.len() == 0 {
                break;
            }

            $pos_counter += $string.len() - trimmed_string.len();
            $string = trimmed_string;
            $(
                match $regex.find($string) {
                    Some((0, n)) => {
                        $container.push(($pos_counter, $token_creator((&$string[0..n]))));
                        $pos_counter += n;
                        $string = &$string[n..];
                        continue;
                    },
                    _ => {}
                }
            )*
            return Err(($pos_counter, "Cannot tokenize".to_string()));
        }
    }};
}

pub fn tokenize_str(mut input: &str)
                    -> Result<Vec<(SourcePosition, Token)>, (SourcePosition, String)> {
    let mut token_container: Vec<(usize, Token)> = Vec::new();
    let mut pos_counter: usize = 0;

    tokenize!(input,
              token_container,
              pos_counter,
              KEYWORD_LET_RX | |_| Token::KeywordLet,
              KEYWORD_FN_RX | |_| Token::KeywordFn,
              KEYWORD_STRUCT_RX | |_| Token::KeywordStruct,
              KEYWORD_WHILE_RX | |_| Token::Keywordwhile,
              KEYWORD_FOR_RX | |_| Token::KeywordFor,
              STRING_LITERAL_RX |
              |input: &str| Token::StringLiteral(input[1..input.len() - 1].to_string()),
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
              COMMA_SEPARATOR_RX | |_| Token::CommaSeparator);
    Ok(token_container)
}

fn line_column_from_pos(pos: SourcePosition, input: &str) -> SourcePositionRowColumn {
    let mut line_count: usize = 0;
    let mut column_count: usize = 0;
    for c in input[..pos].chars() {
        if c == '\n' {
            line_count += 1;
            column_count = 0;
        } else {
            column_count += 1;
        }
    }
    SourcePositionRowColumn(line_count, column_count)
}

#[cfg(test)]
mod tests;
