pub struct Program {
    pub main: Function
}

pub struct Function {
    pub name: String,
    pub body: Return
}

pub struct Return {
    pub return_value: Expr
}

pub enum Expr {
    Const { int: Int },
    Expression { op: UnaryOp, expr: Box<Expr> }
}

pub struct UnaryOp {
    pub op: String
}

pub struct Int {
    pub val: u32
}