mod lexer;

enum Statement {
    Definition(Definition),
    If(BooleanExpression, Vec<Statement>),
    While(BooleanExpression, Vec<Statement>),
    ArithmeticExpression(ArithmeticExpression),
    BooleanExpression(BooleanExpression),
    Return(Object)
}

enum Definition {
    Identifier(String),
    Object(Object)
}

enum Object {
    StringLiteral(String),
    DoubleLiteral(f64),
    BooleanLiteral(bool),
    StructDefinition(Vec<(String, Object)>),
    FunctionDefinition(Vec<Statement>),
    Reference(String)
}

enum BooleanExpression {
    And(Box<BooleanExpression>, Box<BooleanExpression>),
    Or(Box<BooleanExpression>, Box<BooleanExpression>),
    Not(Box<BooleanExpression>),
    Literal(bool)
}

enum ArithmeticExpression {
    Addition(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Subtraction(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Multiplication(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Division(Box<ArithmeticExpression>, Box<ArithmeticExpression>)
}
