#![allow(dead_code)] // ToDo: Remove when not needed anymore.
#![allow(unused_variables)] // ToDo: Remove when not needed anymore.
#![allow(unused_must_use)] // ToDo: Remove when not needed anymore.

pub mod lexer;

use self::lexer::Token;

enum Statement {
    Definition(Definition),
    If(BooleanExpression, Vec<Statement>),
    While(BooleanExpression, Vec<Statement>),
    For(Object, Vec<Statement>),
    ArithmeticExpression(ArithmeticExpression),
    BooleanExpression(BooleanExpression),
    Return(Object),
}

enum Definition {
    Identifier(String),
    Object(Object),
}

enum Object {
    StringLiteral(String),
    DoubleLiteral(f64),
    BooleanLiteral(bool),
    StructDefinition(Vec<(String, Object)>),
    FunctionDefinition(Vec<Statement>),
    Reference(String),
}

enum BooleanExpression {
    And(Box<BooleanExpression>, Box<BooleanExpression>),
    Or(Box<BooleanExpression>, Box<BooleanExpression>),
    Not(Box<BooleanExpression>),
    Literal(Object),
}

enum ArithmeticExpression {
    Addition(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Subtraction(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Multiplication(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Division(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    DoubleLiteral(Object),
}

fn parse_tokens(tokens: &[Token]) -> Vec<Statement> {
    Vec::<Statement>::new()
}

fn parse_statement(tokens: &[Token]) -> Statement {
    // match tokens[0] {
    //     Token::KeywordLet => Statement::Definition(parse_definition(&tokens[1..]));
    //     _ => Statement::Return(Object::StringLiteral("kek".to_string()));
    // }
    Statement::Return(Object::StringLiteral("kek".to_string()))
}

fn parse_definition(tokens: &[Token]) -> Definition {
    Definition::Object(Object::StringLiteral("kek".to_string()))
}
