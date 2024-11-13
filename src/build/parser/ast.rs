#[derive(Debug)]
pub enum Expr {
    Integer(String),
    String(String),
    Identify(String),

    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
    UnaryOp(Box<Expr>, UnaryOperator, Box<Expr>),

    Call(String, Vec<Expr>),
    
}

#[derive(Debug)]
pub enum BinaryOperator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

#[derive(Debug)]
pub enum UnaryOperator{
    Not,
    Negate,
}

#[derive(Debug)]
pub enum Statement {
    AsigmentStmt(String, Expr),
    ReturnStmt(Expr),
    IfStmt(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>),
    FunctionDecl(String, Vec<Statement>, Vec<Box<Statement>>),
    VariableDecl(String, Option<Expr>),
    ExprStm(Expr),
}


#[derive(Debug)]
pub enum Program {
    Statements(Vec<Statement>),
}
